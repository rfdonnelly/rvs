use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashSet;

pub struct ImportPaths {
    /// Keeps track of all files that have been imported
    ///
    /// Used to ensure idempotency of `import`.
    imported_paths: HashSet<PathBuf>,

    /// Stack of required files
    ///
    /// Push on entering `import`, pop on leaving `import`.  Used to determine source-relative
    /// path.
    stack: Vec<PathBuf>,

    /// Search path for `require`
    search_paths: Vec<PathBuf>,
}

impl ImportPaths {
    pub fn new() -> ImportPaths {
        ImportPaths {
            imported_paths: HashSet::new(),
            stack: Vec::new(),
            search_paths: Vec::new(),
        }
    }

    /// Returns true on first call for a given path, false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use rvs_parser::ImportPaths;
    ///
    /// let mut paths = ImportPaths::new();
    /// let path = Path::new("./foo/bar.txt");
    /// if paths.enter_import(path) {
    ///     // ...
    ///     paths.leave_import();
    /// }
    /// ```
    pub fn enter_import(&mut self, path: &Path) -> bool {
        if self.imported_paths.contains(path) {
            false
        } else {
            self.imported_paths.insert(path.to_path_buf());
            self.stack.push(path.to_path_buf());

            true
        }
    }

    pub fn leave_import(&mut self) {
        self.stack.pop();
    }

    /// Returns a path if file found in search path.  Returns an std::io::Error otherwise.
    pub fn find(&self, path: &Path) -> io::Result<PathBuf> {
        // Absolute path
        if path.is_absolute() {
            if path.exists() {
                Ok(path.to_path_buf())
            } else {
                Err(io::Error::new(io::ErrorKind::NotFound,
                                   "File not found"))
            }
        } else {
            // Relative to current source file
            if let Some(current) = self.stack.last() {
                let parent = current.parent().unwrap().join(path);
                if parent.exists() {
                    return Ok(parent);
                }
            }

            // Relative to search path
            let result = self.search_paths.iter()
                .map(|ref p| p.join(path))
                .find(|ref p| p.exists());

            match result {
                Some(path) => Ok(path),
                None => {
                    Err(io::Error::new(io::ErrorKind::NotFound,
                                       "File not found"))
                }
            }
        }
    }

    /// Sets the search path used for `import`
    ///
    /// The string must be a colon separated list of paths.
    ///
    /// # Errors
    ///
    /// Error will be returned for parsed paths that do not exist.  If the search path string contains
    /// a mix of paths that do and do not exist, the paths that do exist will be added to the internal
    /// search path.
    pub fn set_search_path(&mut self, paths: &str) -> io::Result<()> {
        let mut error_paths: Vec<PathBuf> = Vec::new();

        for path in paths.split(':') {
            let path = Path::new(path);

            if path.exists() {
                self.add_search_path(&path);
            } else {
                error_paths.push(path.to_path_buf());
            }
        }

        if error_paths.len() > 0 {
            Err(io::Error::new(io::ErrorKind::NotFound,
                               format!("Paths not found:\n{}",
                                       error_paths.iter()
                                       .map(|path| format!("   {:?}", path))
                                       .collect::<Vec<String>>()
                                       .join("\n"))))
        } else {
            Ok(())
        }
    }

    pub fn add_search_path(&mut self, path: &Path) {
        self.search_paths.push(path.to_path_buf());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn basic() {
        let mut paths = ImportPaths::new();
        let path_str = "../examples";
        let path_dir = fs::canonicalize(path_str).unwrap();
        let path_file = path_dir.join("readme.rvs");

        paths.add_search_path(&path_dir);

        assert_eq!(paths.find(&Path::new("readme.rvs")).unwrap(), path_file);
        assert_eq!(paths.enter_import(&path_file), true)
    }
}
