use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "scss.pest"]
pub struct ScssParser;


#[test]
fn test_class() {
    assert!(ScssParser::parse(Rule::class, "").is_err());
    assert!(ScssParser::parse(Rule::class, "a").is_err());
    assert!(ScssParser::parse(Rule::class, ".").is_err());
    assert!(ScssParser::parse(Rule::class, ".a").is_ok());
}