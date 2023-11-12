use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "scss.pest"]
pub struct ScssParser;


#[test]
fn test_literal_number() {
  assert!(ScssParser::parse(Rule::literal_number, "").is_err());
  assert!(ScssParser::parse(Rule::literal_number, "a").is_err());
  assert!(ScssParser::parse(Rule::literal_number, "1").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, ".1").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, "-.1").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, "-2.1").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, "1.1px").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, ".1px").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, "-.1px").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, "-2.1px").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, "-2.1e10").is_ok());
  assert!(ScssParser::parse(Rule::literal_number, "-2.1e10s").is_ok());
}


#[test]
fn test_literal_string_quoted() {
  assert!(ScssParser::parse(Rule::literal_string_quoted, "").is_err());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"").is_err());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"\\\"\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"abcde\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"abcde''\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"#{$a}\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"#{$a}aaabbbccc\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "''").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'abcde'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'abcde\"\"'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'abcde\"\"\''").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'#{$a}'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'#{$a}aaabbbccc'").is_ok());
}