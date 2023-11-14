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
  assert!(ScssParser::parse(Rule::literal_string_quoted, "\"aaaaa#{$a}aaabbbccc\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "''").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'abcde'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'abcde\"\"'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'abcde\\''").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'abced#{$a}aaaaaa'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'#{$a}aaabbbccc'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_quoted, "'\\1F46D'").is_ok());
}

#[test]
fn test_literal_string_unquoted() {
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "").is_err());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "\"").is_err());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "\"\"").is_err());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "''").is_err());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "abcde").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "-abcde").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "--abcde").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "abcde\\").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "abced#{$a}aaaaaa").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "-#{$a}aaab_b-bccc").is_ok());
  assert!(ScssParser::parse(Rule::literal_string_unquoted, "-#{$prefix}-flex").is_ok());
}


#[test]
fn test_literal_string() {
  assert!(ScssParser::parse(Rule::literal_string, "").is_err());
  assert!(ScssParser::parse(Rule::literal_string, "\"").is_err());
  assert!(ScssParser::parse(Rule::literal_string, "\"\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\"\\\"\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\"abcde\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\"abcde''\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\"#{$a}\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\"aaaaa#{$a}aaabbbccc\"").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "''").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "'abcde'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "'abcde\"\"'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "'abcde\\''").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "'abced#{$a}aaaaaa'").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "'#{$a}aaabbbccc'").is_ok());

  assert!(ScssParser::parse(Rule::literal_string, "abcde").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "-abcde").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "--abcde").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "abcde\\").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "abced#{$a}aaaaaa").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "-abc#{$a}aaab_b-bccc").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "bold").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "-webkit-flex").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "--123").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "ms").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "-#{$prefix}-flex").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\\1F46D").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\\21").is_ok());
  assert!(ScssParser::parse(Rule::literal_string, "\\7Fx").is_ok());
}