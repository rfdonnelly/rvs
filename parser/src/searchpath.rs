use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct SearchPath {
    /// Search path for `import`
    paths: Vec<PathBuf>,
}

impl SearchPath {
    pub fn new(paths: Vec<PathBuf>) -> SearchPath {
        SearchPath { paths }
    }

    /// Sets the search path used for `import`
    ///
    /// The string must be a colon separated list of paths.
    ///
    /// # Errors
    ///
    /// An error will be returned if any of the parsed paths do not exist.  If the search path
    /// string contains a mix of paths that do and do not exist, none of the paths will be added to
    /// the internal search path.
    pub fn from_string(s: &str) -> io::Result<SearchPath> {
        #[cfg(windows)]
        let separator = ';';

        #[cfg(not(windows))]
        let separator = ':';

        let paths: Vec<PathBuf> = s
            .split(separator)
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| Path::new(s).to_path_buf())
            .collect();

        let error_paths: Vec<PathBuf> = paths
            .iter()
            .filter(|path| !path.exists())
            .map(|path| path.clone())
            .collect();

        if !error_paths.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Paths not found:\n{}",
                    error_paths
                        .iter()
                        .map(|path| format!("   {:?}", path))
                        .collect::<Vec<String>>()
                        .join("\n")
                ),
            ))
        } else {
            Ok(SearchPath::new(paths))
        }
    }

    pub fn find(&self, path: &Path) -> io::Result<PathBuf> {
        if path.is_absolute() {
            if path.exists() {
                Ok(path.to_path_buf())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Path '{:?}' does not exist", path),
                ))
            }
        } else {
            let result = self.paths.iter().map(|p| p.join(path)).find(|p| p.exists());

            match result {
                Some(path) => Ok(path),
                None => Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Path '{:?}' not found in: {:?}", path, self.paths),
                )),
            }
        }
    }
}
