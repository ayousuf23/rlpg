use crate::{file_parser::{FileParserErrorKind, FileParser, FileParserError, Rule}, NFA, dfa_builder::DFABuilder, dfa_simulator::DFASimulator};

fn file_parse(filename: &str) -> Result<Vec<Rule>, FileParserError>
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
    let kind = result.err().unwrap().kind;
    println!("{:?}", kind);
    assert!(kind == error_kind);
}

fn assert_regex_build_failure(filename: &str)
{
    let result = file_parse(filename);
    unsafe {
        let build_result = NFA::build_from_rules(&result.unwrap());
        assert!(build_result.is_err());
    }
}

unsafe fn assert_regex(filename: &str, to_produce_token: &Vec<&str>, to_not_produce_token: &Vec<&str>, to_reject: &Vec<&str>, expected_tokens: &Vec<&str>)
{
    let result = file_parse(filename);
    assert!(!result.is_err());
    unsafe {
        let build_result = NFA::build_from_rules(&result.unwrap());
        assert!(!build_result.is_err());
        let nfa = build_result.unwrap();

        println!("here");

        // Simulate each to_accept string on nfa
        let mut i = 0;
        for item in to_produce_token
        {
            let (result, tokens) = nfa.simulate_and_get_token(item);
            assert!(result);
            assert!(tokens[0].name == expected_tokens[i]);
            i += 1;
        }

        println!("here");
        for item in to_not_produce_token
        {
            let (result, tokens) = nfa.simulate_and_get_token(item);
            assert!(result);
            assert!(tokens.len() == 0);
        }

        println!("here");
        for item in to_reject
        {
            let (result, tokens) = nfa.simulate_and_get_token(item);
            assert!(!result);
            assert!(tokens.len() == 0);
        }

        // Get DFA
        println!("here");
        let dfa = DFABuilder::convert_nfa_to_dfa(nfa);
        i = 0;
        for item in to_produce_token
        {
            let (result, tokens) = DFASimulator::simulate_dfa_and_get_tokens(dfa, item);
            assert!(result);
            assert!(tokens[0] == expected_tokens[i]);
            i += 1;
        }

        println!("here");
        for item in to_not_produce_token
        {
            let (result, tokens) = DFASimulator::simulate_dfa_and_get_tokens(dfa, item);
            assert!(result);
            //println!("{:?}", tokens);
            assert!(tokens.len() == 0);
        }

        println!("here");
        for item in to_reject
        {
            let (result, tokens) = DFASimulator::simulate_dfa_and_get_tokens(dfa, item);
            assert!(!result);
            assert!(tokens.len() == 0);
        }
    }
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
fn test_rule_regex()
{
    assert_file_parse_failure("invalid_rule_regex.txt", FileParserErrorKind::InvalidRegex);
    assert_regex_build_failure("invalid_rule_regex2.txt");
    assert_regex_build_failure("invalid_rule_regex3.txt");

    unsafe {
        // Test the right NFA is produced
        let to_produce_tokens = vec!["hello"];
        let to_not_produce_tokens = vec![];
        let to_reject = vec!["", " ", "hell", "    "];
        let tokens = vec!["rule1"];
        assert_regex("valid_rule_regex.txt", &to_produce_tokens, &to_not_produce_tokens, &to_reject, &tokens);
        
        let to_produce_tokens = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "12345", "57383", "   123", "56   "];
        let to_not_produce_tokens = vec![" ", "    ", "     "];
        let to_reject = vec!["", "a"];
        let tokens = vec!["number", "number", "number", "number", "number", "number", "number", "number", "number", "number", "number", "number", "number", "number"];
        assert_regex("valid_rule_regex2.txt", &to_produce_tokens, &to_not_produce_tokens, &to_reject, &tokens);
    }
}

#[test]
fn test_rule_precedence()
{
    unsafe {
        let to_produce_tokens = vec!["hello"];
        let to_not_produce_tokens = vec![];
        let to_reject = vec!["", " ", "hell", "    "];
        let tokens = vec!["rule1"];
        assert_regex("rule_precedence.txt", &to_produce_tokens, &to_not_produce_tokens, &to_reject, &tokens);
        assert_regex("rule_precedence2.txt", &to_produce_tokens, &to_not_produce_tokens, &to_reject, &tokens);

        let to_produce_tokens = vec![];
        let to_not_produce_tokens = vec!["hello"];
        let to_reject = vec!["", " ", "hell", "    "];
        let tokens = vec![];
        assert_regex("rule_precedence3.txt", &to_produce_tokens, &to_not_produce_tokens, &to_reject, &tokens);
    }
}

#[test]
fn test_rule_action_code()
{
    assert_file_parse_failure("invalid_action_code.txt", FileParserErrorKind::InvalidActionCode);
    assert_file_parse_failure("invalid_action_code2.txt", FileParserErrorKind::InvalidActionCode);
    
    assert_file_parse_success("valid_action_code.txt");
    assert_file_parse_success("valid_action_code2.txt");
    assert_file_parse_success("valid_action_code3.txt");
    assert_file_parse_success("valid_action_code4.txt");
}