#[allow(dead_code)]
pub fn parse(s: &str) -> String {
    format!("{:?}", ::rvs_parser::parse(s, &mut ::rvs_parser::SearchPath::new()).unwrap())
}

#[allow(dead_code)]
pub fn parse_result(s: &str) -> Result<(), ::rvs_parser::error::Error> {
    ::rvs_parser::parse(s, &mut ::rvs_parser::SearchPath::new())?;

    Ok(())
}

