use crate::file_parser::{FileParserErrorKind, FileParser, FileParserError};

fn file_parse(filename: &str) -> Result<bool, FileParserError>
{
    let path = "/Users/abdullah/Developer/rlpg/src/tests/file_parser_tests_resources/";
    let file_path = std::path::Path::new(path).join(filename);

    let mut parser = FileParser {
        rules: Vec::new(),
    };

    return parser.parse_file(file_path.as_path().to_str().unwrap());
}

fn assert_file_parse_success(filename: &str)
{
    let result = file_parse(filename);
    assert!(!result.is_err());
} 

fn assert_file_parse_failure(filename: &str, error_kind: FileParserErrorKind)
{
    let result = file_parse(filename);
    assert!(result.is_err());
    assert!(result.err().unwrap().kind == error_kind);
}   


#[test]
fn test_section_header()
{
    let invalid_section_header = "invalid_section_header.txt";
    assert_file_parse_failure(invalid_section_header, FileParserErrorKind::FileDoesNotBeginWithSectionHeader);
    assert_file_parse_failure("invalid_section_header2.txt", FileParserErrorKind::FileDoesNotBeginWithSectionHeader);
    assert_file_parse_failure("invalid_section_header3.txt", FileParserErrorKind::FileDoesNotBeginWithSectionHeader);

    let valid_section_header = "valid_section_header.txt";
    assert_file_parse_success(valid_section_header);
   
}

#[test]
fn test_empty_rules()
{
    assert_file_parse_failure("no_rules.txt", FileParserErrorKind::NoRules);
}

#[test]
fn test_named_rules_with_same_name()
{
    assert_file_parse_failure("duplicate_named_rules.txt", FileParserErrorKind::DuplicateName);
}

#[test]
fn test_invalid_rule_names()
{
    // TODO
    //assert_file_parse_failure("duplicate_named_rules.txt", FileParserErrorKind::DuplicateName);
}

#[test]
fn test_invalid_rule_regex()
{
    // TODO
    //assert_file_parse_failure("duplicate_named_rules.txt", FileParserErrorKind::DuplicateName);
}

#[test]
fn test_invalid_rule_action_code()
{
    // TODO
    //assert_file_parse_failure("duplicate_named_rules.txt", FileParserErrorKind::DuplicateName);
}

#[test]
fn test_rule_parsing()
{
    // TODO
    //assert_file_parse_failure("duplicate_named_rules.txt", FileParserErrorKind::DuplicateName);
}