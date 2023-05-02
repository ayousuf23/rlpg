use crate::{file_parser::{FileParserErrorKind, FileParser, FileParserError, Rule}, NFA, dfa_builder::DFABuilder, dfa_simulator::DFASimulator};

fn file_parse(filename: &str) -> Result<Vec<Rule>, FileParserError>
{
    let path = "/Users/abdullah/Developer/rlpg/src/tests/file_parser_tests_resources/";
    let file_path = std::path::Path::new(path).join(filename);

    let mut parser = FileParser::new();

    return parser.parse_file(file_path.as_path().to_str().unwrap());
}

fn assert_file_parse_success(filename: &str)
{
    let result = file_parse(filename);
    if result.is_err() {
        println!("{:?}", result.err().unwrap().kind);
        assert!(false);
    } else {
        assert!(!result.is_err());
    }
    
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
            println!("{}", i);
            assert!(result);
            assert!(tokens[0].symbol.name == expected_tokens[i]);
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
            println!("{}", i);
            let (result, tokens) = DFASimulator::simulate_dfa_and_get_tokens(dfa, item);
            println!("{}", result);
            assert!(result);
            assert!(tokens[0].name == expected_tokens[i]);
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
    assert_file_parse_failure("invalid_rule_regex4.txt", FileParserErrorKind::InvalidActionCode);

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


#[test]
fn test_section_as_lexer_rule_name()
{
    assert_file_parse_failure("section_as_named_rule.txt", FileParserErrorKind::InvalidRuleName);
}

#[test]
fn test_eof_as_lexer_rule_name()
{
    assert_file_parse_failure("eof_as_named_rule.txt", FileParserErrorKind::InvalidRuleName);
}

#[test]
fn test_root_as_lexer_rule_name()
{
    assert_file_parse_failure("root_as_named_rule.txt", FileParserErrorKind::InvalidRuleName);
}

// Grammar section tests

#[test]
fn test_empty_lexer_followed_by_empty_grammar_section()
{
    assert_file_parse_failure("empty_lexer_grammar.txt", FileParserErrorKind::NoRules);
}


#[test]
fn test_empty_grammar_section()
{
    assert_file_parse_failure("grammar_tests/empty_grammar_section.txt", FileParserErrorKind::NoGrammarRules);
}

#[test]
fn test_duplicate_grammar_rule_names_fails()
{
    assert_file_parse_failure("grammar_tests/duplicate_rule_name.txt", FileParserErrorKind::DuplicateGrammarRuleName);
}

#[test]
fn test_productions_with_new_symbols()
{
    assert_file_parse_failure("grammar_tests/prod_with_new_symbol1.txt", FileParserErrorKind::UnknownSymbol);
    assert_file_parse_failure("grammar_tests/prod_with_new_symbol2.txt", FileParserErrorKind::UnknownSymbol);
    assert_file_parse_failure("grammar_tests/prod_with_new_symbol3.txt", FileParserErrorKind::UnknownSymbol);
}


#[test]
fn test_rule_missing_semicolon()
{
    assert_file_parse_failure("grammar_tests/rule_missing_semicolon.txt", FileParserErrorKind::MissingGrammarRuleEndSymbol);
}

 
#[test]
fn test_rule_formatting()
{
    assert_file_parse_failure("grammar_tests/text_after_semicolon.txt", FileParserErrorKind::InvalidGrammarRule);
    assert_file_parse_failure("grammar_tests/prod_contain_semicolon.txt", FileParserErrorKind::InvalidIdentifier);
    assert_file_parse_failure("grammar_tests/prod_missing_begin_symbol.txt", FileParserErrorKind::InvalidProduction);

    assert_file_parse_failure("grammar_tests/text_before_begin_symbol.txt", FileParserErrorKind::InvalidProduction);
    assert_file_parse_failure("grammar_tests/whitespace_before_begin_symbol.txt", FileParserErrorKind::InvalidProduction);

    assert_file_parse_failure("grammar_tests/empty_production.txt", FileParserErrorKind::InvalidProduction);

    // Space before/after colon
    assert_file_parse_failure("grammar_tests/whitespace_before_colon.txt", FileParserErrorKind::InvalidGrammarRule);
    // Should not result in an error
    assert_file_parse_success("grammar_tests/whitespace_after_colon.txt");

    // Rule name with special characters and ;
    // Rule name with ;
    assert_file_parse_failure("grammar_tests/special_rule_name1.txt", FileParserErrorKind::InvalidIdentifier);
    assert_file_parse_failure("grammar_tests/special_rule_name2.txt", FileParserErrorKind::InvalidIdentifier);
    assert_file_parse_failure("grammar_tests/special_rule_name3.txt", FileParserErrorKind::InvalidIdentifier);

    // Semicolon by itself
    assert_file_parse_failure("grammar_tests/semicolon_by_itself.txt", FileParserErrorKind::InvalidIdentifier);

    // Production by itself
    assert_file_parse_failure("grammar_tests/prod_by_itself.txt", FileParserErrorKind::InvalidIdentifier);
    assert_file_parse_failure("grammar_tests/prod_by_itself2.txt", FileParserErrorKind::InvalidIdentifier);

    // Empty line between productions
    assert_file_parse_failure("grammar_tests/empty_lines_between_prod.txt", FileParserErrorKind::InvalidProduction);
    assert_file_parse_failure("grammar_tests/empty_lines_between_prod2.txt", FileParserErrorKind::InvalidProduction);

    // No : after name
    assert_file_parse_failure("grammar_tests/no_colon_after_name.txt", FileParserErrorKind::InvalidGrammarRule);

    // Consecutive first lines
    assert_file_parse_failure("grammar_tests/consecutive_first_lines.txt", FileParserErrorKind::InvalidProduction);
}

#[test]
fn test_rule_with_duplicate_productions()
{
    assert_file_parse_failure("grammar_tests/duplicate_production.txt", FileParserErrorKind::DuplicateProduction);
}

#[test]
fn test_root_rule_in_grammar_section()
{
    // Test ensure file parse fails if root rule does not exist
    assert_file_parse_failure("grammar_tests/root_rule_missing.txt", FileParserErrorKind::RootRuleDoesNotExist);

    // Test ensure file parse succeeds if root rule exists
    assert_file_parse_success("grammar_tests/root_rule_exists.txt")
}

#[test]
fn test_rule_named_eof_fails()
{
    // Test ensure file parse fails if root rule does not exist
    assert_file_parse_failure("grammar_tests/rule_named_eof.txt", FileParserErrorKind::InvalidIdentifier);
}