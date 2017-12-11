include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;

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
