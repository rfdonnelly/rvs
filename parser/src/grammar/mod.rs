include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    mod expr {
        use super::*;

        #[test]
        fn good() {
            assert_eq!(format!("{:?}", items("a = (5);", &mut ImportPaths::new()).unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(5)))]");
            assert_eq!(format!("{:?}", items("a = 5;", &mut ImportPaths::new()).unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(5)))]");
            assert_eq!(format!("{:?}", items("a = 0xa;", &mut ImportPaths::new()).unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(10)))]");
            assert_eq!(format!("{:?}", items("a = 0xaf;", &mut ImportPaths::new()).unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(175)))]");
        }

        #[test]
        fn bad() {
            assert!(items("a = (5));", &mut ImportPaths::new()).is_err());
            assert!(items("a = (5;", &mut ImportPaths::new()).is_err());
        }

        #[test]
        fn operations() {
            assert_eq!(format!("{:?}", items("a = 1+2;", &mut ImportPaths::new()).unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), BinaryOperation(Number(1), Add, Number(2))))]");

            assert_eq!(format!("{:?}", items("a = 1+2*3;", &mut ImportPaths::new()).unwrap()),
                "[Single(Assignment(Identifier(\"a\"), BinaryOperation(Number(1), Add, BinaryOperation(Number(2), Mul, Number(3)))))]");
        }
    }

    mod identifier {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 0;", &mut ImportPaths::new()).is_ok());
            assert!(items("a_ = 0;", &mut ImportPaths::new()).is_ok());
            assert!(items("__ = 0;", &mut ImportPaths::new()).is_ok());
            assert!(items("_0 = 0;", &mut ImportPaths::new()).is_ok());
            assert!(items("a::B = 0;", &mut ImportPaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a-b = 0;", &mut ImportPaths::new()).is_err());
            assert!(items("0b = 0;", &mut ImportPaths::new()).is_err());
            assert!(items("1_ = 0;", &mut ImportPaths::new()).is_err());
        }
    }

    mod number {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 5;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 5_;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 5_6;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 5__6;", &mut ImportPaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a = 0b0;", &mut ImportPaths::new()).is_err());
            assert!(items("a = 1z;", &mut ImportPaths::new()).is_err());
        }
    }

    mod hex_number {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 0xa5E4;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 0XA5;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 0X_A5;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 0XA_5;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 0XA__5;", &mut ImportPaths::new()).is_ok());
            assert!(items("a = 0XA5_;", &mut ImportPaths::new()).is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a = 0xg5;", &mut ImportPaths::new()).is_err());
        }
    }

    mod assignment {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a=5;", &mut ImportPaths::new()).is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(format!("{:?}", items("a=5;", &mut ImportPaths::new()).unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(5)))]");
        }

        #[test]
        fn bad() {
            assert!(items("a=5", &mut ImportPaths::new()).is_err());
        }

        #[test]
        fn with_enum() {
            assert!(items("a = Enum::Value;", &mut ImportPaths::new()).is_ok());
        }
    }

    mod items {
        use super::*;

        #[test]
        fn good() {
            assert!(items(" a  = 5 ; \nb=6;", &mut ImportPaths::new()).is_ok());
        }

        #[test]
        fn expr_whitespace() {
            assert!(items("a = 5 + 6 | 10 * ( 5 ^ 3) ;", &mut ImportPaths::new()).is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(format!("{:?}", items(" a  = // comment0\n5 ; // comment1\nb=6;", &mut ImportPaths::new()).unwrap()),
            "[Single(Assignment(Identifier(\"a\"), Number(5))), Single(Assignment(Identifier(\"b\"), Number(6)))]");
        }
    }

    mod range {
        use super::*;

        #[test]
        fn ast() {
            assert_eq!(format!("{:?}", items("a = [1,2];", &mut ImportPaths::new()).unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Function(Range, [Number(1), Number(2)])))]");
        }
    }
}
