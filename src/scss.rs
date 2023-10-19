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
  assert!(ScssParser::parse(Rule::class, "#abc-a_ad").is_err());
  assert!(ScssParser::parse(Rule::class, ".a").is_ok());
  assert!(ScssParser::parse(Rule::class, ".abc-a_ad").is_ok());
}

#[test]
fn test_id() {
  assert!(ScssParser::parse(Rule::id, "").is_err());
  assert!(ScssParser::parse(Rule::id, "a").is_err());
  assert!(ScssParser::parse(Rule::id, "#").is_err());
  assert!(ScssParser::parse(Rule::id, ".abc-a_ad").is_err());
  assert!(ScssParser::parse(Rule::id, "#a").is_ok());
  assert!(ScssParser::parse(Rule::id, "#abc-a_ad").is_ok());
}

#[test]
fn test_identifier() {
  assert!(ScssParser::parse(Rule::identifier, "").is_err());
  assert!(ScssParser::parse(Rule::identifier, "#").is_err());
  assert!(ScssParser::parse(Rule::identifier, ".").is_err());
  assert!(ScssParser::parse(Rule::identifier, "a").is_ok());
  assert!(ScssParser::parse(Rule::identifier, ".abc-a_ad").is_err());
  assert!(ScssParser::parse(Rule::identifier, "a").is_ok());
  assert!(ScssParser::parse(Rule::identifier, "abc-a_ad").is_ok());
}

#[test]
fn test_selector_part() {
  assert!(ScssParser::parse(Rule::selector_part, "").is_err());
  assert!(ScssParser::parse(Rule::selector_part, "#").is_err());
  assert!(ScssParser::parse(Rule::selector_part, ".").is_err());

  assert_eq!(Rule::selector_part, ScssParser::parse(Rule::selector_part, "#abc-a_ad").unwrap().next().unwrap().as_rule());
  assert_eq!(Rule::selector_part, ScssParser::parse(Rule::selector_part, ".abc-a_ad").unwrap().next().unwrap().as_rule());
  assert_eq!(Rule::selector_part, ScssParser::parse(Rule::selector_part, "abc-a_ad").unwrap().next().unwrap().as_rule());
}