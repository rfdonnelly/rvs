include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    mod expr {
        use super::*;

        #[test]
        fn good() {
            assert_eq!(items("a = (5);", &mut RequirePaths::new()).unwrap(), vec![Item::Single(Box::new(
                        Node::Assignment(
                            Box::new(Node::Identifier("a".into())),
                            Box::new(Node::Number(5))
                        )
                    ))]);
            assert_eq!(items("a = 5;", &mut RequirePaths::new()).unwrap(), vec![Item::Single(Box::new(
                        Node::Assignment(
                            Box::new(Node::Identifier("a".into())),
                            Box::new(Node::Number(5))
                        )
                    ))]);
            assert_eq!(items("a = 0xa;", &mut RequirePaths::new()).unwrap(), vec![Item::Single(Box::new(
                        Node::Assignment(
                            Box::new(Node::Identifier("a".into())),
                            Box::new(Node::Number(10))
                        )
                    ))]);
            assert_eq!(items("a = 0xaf;", &mut RequirePaths::new()).unwrap(), vec![Item::Single(Box::new(
                        Node::Assignment(
                            Box::new(Node::Identifier("a".into())),
                            Box::new(Node::Number(0xaf))
                        )
                    ))]);
        }

        #[test]
        fn bad() {
            assert!(items("a = (5));", &mut RequirePaths::new()).is_err());
            assert!(items("a = (5;", &mut RequirePaths::new()).is_err());
        }

        #[test]
        fn operations() {
            assert_eq!(items("a = 1+2;", &mut RequirePaths::new()).unwrap(), vec![
                       Item::Single(
                           Box::new(Node::Assignment(
                                   Box::new(Node::Identifier("a".into())),
                                   Box::new(Node::BinaryOperation(
                                           Box::new(Node::Number(1)),
                                           BinaryOpcode::Add,
                                           Box::new(Node::Number(2))
                                           ))
                                   ))
                           )
            ]);

            assert_eq!(format!("{:?}", items("a = 1+2*3;", &mut RequirePaths::new()).unwrap()),
                "[Single(Assignment(Identifier(\"a\"), BinaryOperation(Number(1), Add, BinaryOperation(Number(2), Mul, Number(3)))))]");
        }
    }

    mod identifier {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 0;", &mut RequirePaths::new()).is_ok());
            assert!(items("a_ = 0;", &mut RequirePaths::new()).is_ok());
            assert!(items("__ = 0;", &mut RequirePaths::new()).is_ok());
            assert!(items("_0 = 0;", &mut RequirePaths::new()).is_ok());
            assert!(items("a::B = 0;", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a-b = 0;", &mut RequirePaths::new()).is_err());
            assert!(items("0b = 0;", &mut RequirePaths::new()).is_err());
            assert!(items("1_ = 0;", &mut RequirePaths::new()).is_err());
        }
    }

    mod number {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 5;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 5_;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 5_6;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 5__6;", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a = a;", &mut RequirePaths::new()).is_err());
            assert!(items("a = _5;", &mut RequirePaths::new()).is_err());
        }
    }

    mod hex_number {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 0xa5E4;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 0XA5;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 0X_A5;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 0XA_5;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 0XA__5;", &mut RequirePaths::new()).is_ok());
            assert!(items("a = 0XA5_;", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a = 0xg5;", &mut RequirePaths::new()).is_err());
        }
    }

    mod assignment {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a=5;", &mut RequirePaths::new()).is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(items("a=5;", &mut RequirePaths::new()).unwrap(), vec![
                Item::Single(
                    Box::new(
                        Node::Assignment(
                            Box::new(Node::Identifier("a".into())),
                            Box::new(Node::Number(5))
                        )
                    )
                )
            ]);
        }

        #[test]
        fn bad() {
            assert!(items("a=5", &mut RequirePaths::new()).is_err());
        }

        #[test]
        fn with_enum() {
            assert!(items("a = Enum::Value;", &mut RequirePaths::new()).is_ok());
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
            assert_eq!(items(" a  = // comment0\n5 ; // comment1\nb=6;", &mut RequirePaths::new()).unwrap(), vec![
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
            ]);
        }
    }

    mod range {
        use super::*;

        #[test]
        fn ast() {
            assert_eq!(items("a = [1,2];", &mut RequirePaths::new()).unwrap(), vec![
                       Item::Single(
                           Box::new(Node::Assignment(
                                   Box::new(Node::Identifier("a".into())),
                                   Box::new(
                                       Node::Function(
                                           Function::Range,
                                           vec![
                                           Box::new(Node::Number(1)),
                                           Box::new(Node::Number(2))
                                           ]
                                           )
                                       )
                                   )))
            ]);
        }
    }
}