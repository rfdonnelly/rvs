use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SearchPath {
    /// Search path for `import`
    pub paths: Vec<PathBuf>,
}

impl SearchPath {
    pub fn new() -> SearchPath {
        SearchPath {
            paths: Vec::new(),
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
    pub fn set(&mut self, paths: &str) -> io::Result<()> {
        let mut error_paths: Vec<PathBuf> = Vec::new();

        for path in paths.split(':') {
            let path = Path::new(path);

            if path.exists() {
                self.add(&path);
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

    pub fn add(&mut self, path: &Path) {
        self.paths.push(path.to_path_buf());
    }
}
