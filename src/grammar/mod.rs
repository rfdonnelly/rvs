include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    mod atom {
        use super::*;

        #[test]
        fn good() {
            assert_eq!(atom("(5)"), Ok(Box::new(Node::Number(5))));
            assert_eq!(atom("5"), Ok(Box::new(Node::Number(5))));
            assert_eq!(atom("0xa"), Ok(Box::new(Node::Number(10))));
            assert_eq!(atom("0xaf"), Ok(Box::new(Node::Number(0xaf))));
        }

        #[test]
        fn bad() {
            assert!(atom("(5))").is_err());
            assert!(atom("(5").is_err());
        }
    }

    mod expr {
        use super::*;

        #[test]
        fn good() {
            assert_eq!(expr("1+2"), Ok(
                Box::new(Node::Operation(
                    Box::new(Node::Number(1)),
                    Opcode::Add,
                    Box::new(Node::Number(2))
                ))
            ));

            assert_eq!(format!("{:?}", expr("1+2*3")),
            "Ok(Operation(Number(1), Add, Operation(Number(2), Mul, Number(3))))");
        }
    }

    mod identifier {
        use super::*;

        #[test]
        fn good() {
            assert!(identifier("a").is_ok());
            assert!(identifier("a_").is_ok());
            assert!(identifier("__").is_ok());
            assert!(identifier("_0").is_ok());
            assert!(identifier("a::B").is_ok());
        }

        #[test]
        fn bad() {
            assert!(identifier("a-b").is_err());
            assert!(identifier("0b").is_err());
            assert!(identifier("1_").is_err());
        }
    }

    mod number {
        use super::*;

        #[test]
        fn good() {
            assert!(number("5").is_ok());
            assert!(number("5_").is_ok());
            assert!(number("5_6").is_ok());
            assert!(number("5__6").is_ok());
        }

        #[test]
        fn bad() {
            assert!(number("a").is_err());
            assert!(number("_5").is_err());
        }
    }

    mod hex_number {
        use super::*;

        #[test]
        fn good() {
            assert!(hex_number("0xa5E4").is_ok());
            assert!(hex_number("0XA5").is_ok());
            assert!(hex_number("0X_A5").is_ok());
            assert!(hex_number("0XA_5").is_ok());
            assert!(hex_number("0XA__5").is_ok());
            assert!(hex_number("0XA5_").is_ok());
        }

        #[test]
        fn bad() {
            assert!(hex_number("0xg5").is_err());
        }
    }

    mod assignment {
        use super::*;

        #[test]
        fn good() {
            assert!(assignment("a=5;").is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(assignment("a=5;"), Ok(
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
            assert!(assignment("a=5").is_err());
        }

        #[test]
        fn with_enum() {
            assert!(assignment("a = Enum::Value;").is_ok());
        }
    }

    mod items {
        use super::*;

        #[test]
        fn good() {
            assert!(items(" a  = 5 ; \nb=6;").is_ok());
        }

        #[test]
        fn expr_whitespace() {
            assert!(items("a = 5 + 6 | 10 * ( 5 ^ 3) ;").is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(items(" a  = // comment0\n5 ; // comment1\nb=6;"), Ok(vec![
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
            assert_eq!(range("[1,2]"), Ok(
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
