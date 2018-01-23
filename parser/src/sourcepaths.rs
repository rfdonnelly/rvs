use searchpath::SearchPath;

use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashSet;

pub struct SourcePaths {
    /// Keeps track of all files that have been imported
    ///
    /// Used to ensure idempotency of `import`.
    paths: HashSet<PathBuf>,

    /// Stack of imported files
    ///
    /// Push on entering `import`, pop on leaving `import`.  Used to determine source-relative
    /// path.
    stack: Vec<PathBuf>,

    searchpath: SearchPath,
}

impl SourcePaths {
    pub fn new(searchpath: SearchPath) -> SourcePaths {
        SourcePaths {
            paths: HashSet::new(),
            stack: Vec::new(),
            searchpath,
        }
    }

    /// Returns true on first call for a given path, false otherwise.
    ///
    /// All enter_import calls that return true must be paired with a leave_import call.
    pub fn enter_import(&mut self, path: &Path) -> bool {
        if self.paths.contains(path) {
            false
        } else {
            self.paths.insert(path.to_path_buf());
            self.stack.push(path.to_path_buf());

            true
        }
    }

    pub fn leave_import(&mut self) {
        self.stack.pop();
    }

    /// Returns a path if file found in search path.  Returns an std::io::Error otherwise.
    pub fn find(&self, path: &Path) -> io::Result<PathBuf> {
        // Relative to current source file
        if let Some(current) = self.stack.last() {
            let parent = current.parent().unwrap().join(path);
            if parent.exists() {
                return Ok(parent);
            }
        }

        // Relative to search path
        let result = self.searchpath
            .paths
            .iter()
            .map(|ref p| p.join(path))
            .find(|ref p| p.exists());

        match result {
            Some(path) => Ok(path),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn basic() {
        let path_str = "../examples";
        let path_dir = fs::canonicalize(path_str).unwrap();
        let path_file = path_dir.join("readme.rvs");

        let search_path = SearchPath::new(vec![path_dir]);
        let mut sourcepaths = SourcePaths::new(search_path);

        assert_eq!(
            sourcepaths.find(&Path::new("readme.rvs")).unwrap(),
            path_file
        );
        assert!(sourcepaths.enter_import(&path_file));
        assert!(!sourcepaths.enter_import(&path_file));
        sourcepaths.leave_import();
        assert!(!sourcepaths.enter_import(&path_file));
    }
}
