#[allow(dead_code)]
pub fn parse(s: &str) -> String {
    let parser = ::rvs_parser::Parser::new(::rvs_parser::SearchPath::new());
    format!("{:?}", parser.parse(s).unwrap())
}

#[allow(dead_code)]
pub fn parse_result(s: &str) -> Result<(), ::rvs_parser::error::Error> {
    let parser = ::rvs_parser::Parser::new(::rvs_parser::SearchPath::new());
    parser.parse(s)?;

    Ok(())
}

