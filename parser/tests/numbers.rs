extern crate rvs_parser;

mod utils;
use utils::*;

mod number {
    use super::*;

    #[test]
    fn good() {
        assert!(parse_result("a = 5;").is_ok());
        assert!(parse_result("a = 5_;").is_ok());
        assert!(parse_result("a = 5_6;").is_ok());
        assert!(parse_result("a = 5__6;").is_ok());
    }

    #[test]
    fn bad() {
        assert!(parse_result("a = 0b0;").is_err());
        assert!(parse_result("a = 1z;").is_err());
    }
}

mod hex_number {
    use super::*;

    #[test]
    fn good() {
        assert!(parse_result("a = 0xa5E4;").is_ok());
        assert!(parse_result("a = 0XA5;").is_ok());
        assert!(parse_result("a = 0X_A5;").is_ok());
        assert!(parse_result("a = 0XA_5;").is_ok());
        assert!(parse_result("a = 0XA__5;").is_ok());
        assert!(parse_result("a = 0XA5_;").is_ok());
    }

    #[test]
    fn bad() {
        assert!(parse_result("a = 0xg5;").is_err());
    }
}
