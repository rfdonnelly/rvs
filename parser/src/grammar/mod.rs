include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    fn items(s: &str) -> ParseResult<Vec<Item>> {
        super::items(s, &mut SearchPath::new())
    }

    mod expr {
        use super::*;

        #[test]
        fn good() {
            assert_eq!(format!("{:?}", items("a = (5);").unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(5)))]");
            assert_eq!(format!("{:?}", items("a = 5;").unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(5)))]");
            assert_eq!(format!("{:?}", items("a = 0xa;").unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(10)))]");
            assert_eq!(format!("{:?}", items("a = 0xaf;").unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(175)))]");
        }

        #[test]
        fn bad() {
            assert!(items("a = (5));").is_err());
            assert!(items("a = (5;").is_err());
        }

        #[test]
        fn operations() {
            assert_eq!(format!("{:?}", items("a = 1+2;").unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), BinaryOperation(Number(1), Add, Number(2))))]");

            assert_eq!(format!("{:?}", items("a = 1+2*3;").unwrap()),
                "[Single(Assignment(Identifier(\"a\"), BinaryOperation(Number(1), Add, BinaryOperation(Number(2), Mul, Number(3)))))]");
        }
    }

    mod identifier {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 0;").is_ok());
            assert!(items("a_ = 0;").is_ok());
            assert!(items("__ = 0;").is_ok());
            assert!(items("_0 = 0;").is_ok());
            assert!(items("a::B = 0;").is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a-b = 0;").is_err());
            assert!(items("0b = 0;").is_err());
            assert!(items("1_ = 0;").is_err());
        }
    }

    mod number {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 5;").is_ok());
            assert!(items("a = 5_;").is_ok());
            assert!(items("a = 5_6;").is_ok());
            assert!(items("a = 5__6;").is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a = 0b0;").is_err());
            assert!(items("a = 1z;").is_err());
        }
    }

    mod hex_number {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a = 0xa5E4;").is_ok());
            assert!(items("a = 0XA5;").is_ok());
            assert!(items("a = 0X_A5;").is_ok());
            assert!(items("a = 0XA_5;").is_ok());
            assert!(items("a = 0XA__5;").is_ok());
            assert!(items("a = 0XA5_;").is_ok());
        }

        #[test]
        fn bad() {
            assert!(items("a = 0xg5;").is_err());
        }
    }

    mod assignment {
        use super::*;

        #[test]
        fn good() {
            assert!(items("a=5;").is_ok());
        }

        #[test]
        fn ast() {
            assert_eq!(format!("{:?}", items("a=5;").unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Number(5)))]");
        }

        #[test]
        fn bad() {
            assert!(items("a=5").is_err());
        }

        #[test]
        fn with_enum() {
            assert!(items("a = Enum::Value;").is_ok());
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
            assert_eq!(format!("{:?}", items(" a  = // comment0\n5 ; // comment1\nb=6;").unwrap()),
            "[Single(Assignment(Identifier(\"a\"), Number(5))), Single(Assignment(Identifier(\"b\"), Number(6)))]");
        }
    }

    mod range {
        use super::*;

        #[test]
        fn ast() {
            assert_eq!(format!("{:?}", items("a = [1,2];").unwrap()),
                       "[Single(Assignment(Identifier(\"a\"), Function(Range, [Number(1), Number(2)])))]");
        }
    }
}
