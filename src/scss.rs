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
  assert!(ScssParser::parse(Rule::class, ". abc-a_ad").is_err());
}

#[test]
fn test_id() {
  assert!(ScssParser::parse(Rule::id, "").is_err());
  assert!(ScssParser::parse(Rule::id, "a").is_err());
  assert!(ScssParser::parse(Rule::id, "#").is_err());
  assert!(ScssParser::parse(Rule::id, ".abc-a_ad").is_err());
  assert!(ScssParser::parse(Rule::id, "#a").is_ok());
  assert!(ScssParser::parse(Rule::id, "#abc-a_ad").is_ok());
  assert!(ScssParser::parse(Rule::id, "# abc-a_ad").is_err());
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
fn test_property() {
  assert!(ScssParser::parse(Rule::property, "").is_err());
  assert!(ScssParser::parse(Rule::property, "#").is_err());
  assert!(ScssParser::parse(Rule::property, ".").is_err());
  assert!(ScssParser::parse(Rule::property, "a").is_ok());
  assert!(ScssParser::parse(Rule::property, ".abc-a_ad").is_err());
  assert!(ScssParser::parse(Rule::property, "a").is_ok());
  assert!(ScssParser::parse(Rule::property, "abc-a_ad").is_ok());
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

#[test]
fn test_selector() {
  assert!(ScssParser::parse(Rule::selector, "").is_err());
  assert!(ScssParser::parse(Rule::selector, "#").is_err());
  assert!(ScssParser::parse(Rule::selector, ".").is_err());

  assert_eq!(Rule::selector, ScssParser::parse(Rule::selector, "#abc-a_ad").unwrap().next().unwrap().as_rule());
  assert_eq!(Rule::selector, ScssParser::parse(Rule::selector, ".abc-a_ad").unwrap().next().unwrap().as_rule());
  assert_eq!(Rule::selector, ScssParser::parse(Rule::selector, "abc-a_ad").unwrap().next().unwrap().as_rule());
  let mut result = ScssParser::parse(Rule::selector, "#abc-a_ad .abc-a_ad\tabc-a_ad\r\n.a").unwrap().next().unwrap().into_inner();
  let pair = result.next().unwrap();
  assert_eq!(Rule::selector_part, pair.as_rule());
  assert_eq!("#abc-a_ad", pair.as_str());
  let pair = result.next().unwrap();
  assert_eq!(Rule::selector_part, pair.as_rule());
  assert_eq!(".abc-a_ad", pair.as_str());
  let pair = result.next().unwrap();
  assert_eq!(Rule::selector_part, pair.as_rule());
  assert_eq!("abc-a_ad", pair.as_str());
  let pair = result.next().unwrap();
  assert_eq!(Rule::selector_part, pair.as_rule());
  assert_eq!(".a", pair.as_str());
}

#[test]
fn test_variable_name() {
  assert!(ScssParser::parse(Rule::variable_name, "").is_err());
  assert!(ScssParser::parse(Rule::variable_name, "#").is_err());
  assert!(ScssParser::parse(Rule::variable_name, ".").is_err());
  assert!(ScssParser::parse(Rule::variable_name, ".a").is_err());
  assert!(ScssParser::parse(Rule::variable_name, "Aaaa").is_err());
  assert!(ScssParser::parse(Rule::variable_name, "1aaa").is_err());
  assert!(ScssParser::parse(Rule::variable_name, "aaaa").is_ok());
  assert!(ScssParser::parse(Rule::variable_name, "aaaa12").is_ok());
  assert!(ScssParser::parse(Rule::variable_name, "aaaa_aa").is_ok());
}

#[test]
fn test_string() {
  assert!(ScssParser::parse(Rule::string, "\"a\"").is_ok());
  assert!(ScssParser::parse(Rule::string, "a").is_err());
  assert!(ScssParser::parse(Rule::string, "\"a").is_err());
  assert!(ScssParser::parse(Rule::string, "a\"").is_err());
}