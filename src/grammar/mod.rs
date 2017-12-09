use std::collections::HashSet;
use std::io;
use std::path::PathBuf;

pub struct RequirePaths {
    /// Keeps track of all files that have been required
    ///
    /// Used to ensure idempotency of `require`.
    required_paths: HashSet<PathBuf>,

    /// Stack of required files
    ///
    /// Push on entering `require`, pop on leaving `require`.  Used to determine source-relative
    /// path.
    stack: Vec<PathBuf>,

    /// Search path for `require`
    search_paths: Vec<PathBuf>,
}

impl RequirePaths {
    pub fn new() -> RequirePaths {
        RequirePaths {
            required_paths: HashSet::new(),
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
    /// use rvs::grammar::RequirePaths;
    ///
    /// let mut paths = RequirePaths::new();
    /// let path = Path::new("./foo/bar.txt");
    /// if paths.enter_require(path) {
    ///     // ...
    ///     paths.leave_require();
    /// }
    /// ```
    pub fn enter_require(&mut self, path: &Path) -> bool {
        if self.required_paths.contains(path) {
            false
        } else {
            self.required_paths.insert(path.to_path_buf());
            self.stack.push(path.to_path_buf());

            true
        }
    }

    pub fn leave_require(&mut self) {
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

    /// Sets the search path used for `require`
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

include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    mod require_paths {
        use super::*;

        use std::fs;
        use std::env;

        #[test]
        fn basic() {
            let mut paths = RequirePaths::new();
            let path_str = "examples";
            let path_dir = fs::canonicalize(path_str).unwrap();
            let path_file = path_dir.join("basic.rvs");

            paths.add_search_path(&path_dir);

            assert_eq!(paths.find(&Path::new("basic.rvs")).unwrap(), path_file);
            assert_eq!(paths.enter_require(&path_file), true)
        }
    }

    mod atom {
        use super::*;

        #[test]
        fn good() {
            assert_eq!(atom("(5)", &mut RequirePaths::new()), Ok(Box::new(Node::Number(5))));
            assert_eq!(atom("5", &mut RequirePaths::new()), Ok(Box::new(Node::Number(5))));
            assert_eq!(atom("0xa", &mut RequirePaths::new()), Ok(Box::new(Node::Number(10))));
            assert_eq!(atom("0xaf", &mut RequirePaths::new()), Ok(Box::new(Node::Number(0xaf))));
        }

        #[test]
        fn bad() {
            assert!(atom("(5))", &mut RequirePaths::new()).is_err());
            assert!(atom("(5", &mut RequirePaths::new()).is_err());
        }
    }

    mod expr {
        use super::*;

        #[test]
        fn good() {
            assert_eq!(expr("1+2", &mut RequirePaths::new()), Ok(
                Box::new(Node::BinaryOperation(
                    Box::new(Node::Number(1)),
                    BinaryOpcode::Add,
                    Box::new(Node::Number(2))
                ))
            ));

            assert_eq!(format!("{:?}", expr("1+2*3", &mut RequirePaths::new())),
            "Ok(BinaryOperation(Number(1), Add, BinaryOperation(Number(2), Mul, Number(3))))");
        }
    }

    mod identifier {
        use super::*;

        #[test]
        fn good() {
            assert!(identifier("a", &mut RequirePaths::new()).is_ok());
            assert!(identifier("a_", &mut RequirePaths::new()).is_ok());
            assert!(identifier("__", &mut RequirePaths::new()).is_ok());
            assert!(identifier("_0", &mut RequirePaths::new()).is_ok());
            assert!(identifier("a::B", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(identifier("a-b", &mut RequirePaths::new()).is_err());
            assert!(identifier("0b", &mut RequirePaths::new()).is_err());
            assert!(identifier("1_", &mut RequirePaths::new()).is_err());
        }
    }

    mod number {
        use super::*;

        #[test]
        fn good() {
            assert!(number("5", &mut RequirePaths::new()).is_ok());
            assert!(number("5_", &mut RequirePaths::new()).is_ok());
            assert!(number("5_6", &mut RequirePaths::new()).is_ok());
            assert!(number("5__6", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(number("a", &mut RequirePaths::new()).is_err());
            assert!(number("_5", &mut RequirePaths::new()).is_err());
        }
    }

    mod hex_number {
        use super::*;

        #[test]
        fn good() {
            assert!(hex_number("0xa5E4", &mut RequirePaths::new()).is_ok());
            assert!(hex_number("0XA5", &mut RequirePaths::new()).is_ok());
            assert!(hex_number("0X_A5", &mut RequirePaths::new()).is_ok());
            assert!(hex_number("0XA_5", &mut RequirePaths::new()).is_ok());
            assert!(hex_number("0XA__5", &mut RequirePaths::new()).is_ok());
            assert!(hex_number("0XA5_", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(hex_number("0xg5", &mut RequirePaths::new()).is_err());
        }
    }

    mod assignment {
        use super::*;

        #[test]
        fn good() {
            assert!(assignment("a=5;", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(assignment("a=5;", &mut RequirePaths::new()), Ok(
                Item::Single(
                    Box::new(
                        Node::Assignment(
                            Box::new(Node::Identifier("a".into())),
                            Box::new(Node::Number(5))
                        )
                    )
                )
            ));
        }

        #[test]
        fn bad() {
            assert!(assignment("a=5", &mut RequirePaths::new()).is_err());
        }

        #[test]
        fn with_enum() {
            assert!(assignment("a = Enum::Value;", &mut RequirePaths::new()).is_ok());
        }
    }

    mod items {
        use super::*;

        #[test]
        fn good() {
            assert!(items(" a  = 5 ; \nb=6;", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn expr_whitespace() {
            assert!(items("a = 5 + 6 | 10 * ( 5 ^ 3) ;", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(items(" a  = // comment0\n5 ; // comment1\nb=6;", &mut RequirePaths::new()), Ok(vec![
                Item::Single(
                    Box::new(Node::Assignment(
                        Box::new(Node::Identifier("a".into())),
                        Box::new(Node::Number(5))
                    )),
                ),
                Item::Single(
                    Box::new(Node::Assignment(
                        Box::new(Node::Identifier("b".into())),
                        Box::new(Node::Number(6))
                    )),
                )
            ]));
        }
    }

    mod range {
        use super::*;

        #[test]
        fn ast() {
            assert_eq!(range("[1,2]", &mut RequirePaths::new()), Ok(
                    Box::new(
                        Node::Function(
                            Function::Range,
                            vec![
                                Box::new(Node::Number(1)),
                                Box::new(Node::Number(2))
                            ]
                        )
                    )
                ));
        }
    }

}
