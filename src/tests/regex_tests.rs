use crate::{NFABuilder, regex_parser::{RegExParser, RegExParserError}, NFA, DFABuilder, dfa_simulator::DFASimulator};

unsafe fn assert_parse_error(regex: &str, kind: RegExParserError)
{
    let mut parser = RegExParser::new(regex);
    let parse_root = parser.parse();
    assert!(parse_root.is_err());
    let err = parse_root.err().unwrap();
    println!("{}", &err);
    assert!(err == kind);
}

unsafe fn get_nfa(regex: &str) -> NFA {
    // Create a regex parser
    let mut parser = RegExParser::new(&regex);
    let parse_root = parser.parse();

    // Generate an NFA
    return NFABuilder::build(&parse_root.unwrap()).expect("Error");
}

unsafe fn test_regex(pattern: &str, to_accept: &Vec<&str>, to_reject: &Vec<&str>)
{
    // Get NFA
    let nfa = get_nfa(pattern);

    // Simulate each to_accept string on nfa
    for item in to_accept
    {
        assert!(nfa.simulate(item));
    }

    for item in to_reject
    {
        assert!(!nfa.simulate(item));
    }

    // Get DFA
    let dfa = DFABuilder::convert_nfa_to_dfa(nfa);
    for item in to_accept
    {
        println!("{}", item);
        assert!(DFASimulator::simulate_dfa(dfa, item));
    }

    for item in to_reject
    {
        assert!(!DFASimulator::simulate_dfa(dfa, item));
    }
}

#[test]
fn fails_on_empty_string()
{
    unsafe
    {
        assert_parse_error("", RegExParserError::EmptyPattern);
    }
}

#[test]
fn any_char_tests()
{
    unsafe
    {
        // Test any char does not accept empty string
        test_regex(".", &vec![], &vec![""]);

        let to_accept = vec!["aa", "ab", "aw", "aq", "a."];
        let to_reject = vec!["a", "b", "bc", "abcd78a", "aaaaaaaab", "aaaaaaaaa"];
        test_regex("a.", &to_accept, &to_reject);

        let to_accept = vec!["aaaa", "bbbb", "hd23", "    "];
        let to_reject = vec!["abcd78a", "aaaaaaaab", ""];
        test_regex("....", &to_accept, &to_reject);
    }
}

#[test]
fn parentheses_tests()
{
    unsafe
    {
        // Test fails on empty parenthesis
        assert_parse_error("()", RegExParserError::InvalidInnerParenthesesExpression);
        assert_parse_error("(", RegExParserError::InvalidInnerParenthesesExpression);
        assert_parse_error(")", RegExParserError::UnmatchedOpenAndCloseParentheses);
        assert_parse_error("a(", RegExParserError::InvalidInnerParenthesesExpression);
        assert_parse_error("a)", RegExParserError::UnmatchedOpenAndCloseParentheses);
        assert_parse_error("a()", RegExParserError::InvalidInnerParenthesesExpression);

        // Test nested parentheses
        let to_accept = vec!["a",];
        let to_reject = vec!["", "b", "abcd78a", "aaaaaaaaa", "a."];
        test_regex("((a))", &to_accept, &to_reject);

        let to_accept = vec!["ab"];
        let to_reject = vec!["", "b", "a", "aaaaaaaaa", "ad", "bbbbbbb"];
        test_regex("(ab)", &to_accept, &to_reject);
    }
}

#[test]
fn plus_tests()
{
    unsafe
    {
        assert_parse_error("+", RegExParserError::CharacterMustBeEscaped);

        let to_accept = vec!["a", "aa", "aaa", "aaaaaaaaaaa"];
        let to_reject = vec!["", "b", "bc", "abcd78a", "aaaaaaaab", "a."];
        test_regex("a+", &to_accept, &to_reject);

        test_regex("a+++++", &to_accept, &to_reject);
    }
}

#[test]
fn kleene_star_tests()
{
    unsafe
    {
        assert_parse_error("*", RegExParserError::CharacterMustBeEscaped);

        let to_accept = vec!["", "a", "aa", "aaa", "aaaaaaaaaaa"];
        let to_reject = vec!["b", "aaaaaaaab", "a."];
        test_regex("a*", &to_accept, &to_reject);

        test_regex("a****", &to_accept, &to_reject);
    }
}

#[test]
fn question_mark_tests()
{
    unsafe
    {
        assert_parse_error("?", RegExParserError::CharacterMustBeEscaped);

        let to_accept = vec!["", "a"];
        let to_reject = vec!["b", "aa", "aaaa", "aaaaaaa"];
        test_regex("a?", &to_accept, &to_reject);
        test_regex("a????", &to_accept, &to_reject);

        let to_accept = vec!["b", "ab"];
        let to_reject = vec!["", "a", "aa", "bb"];
        test_regex("a?b", &to_accept, &to_reject);
    }
}

#[test]
fn brackets_tests()
{
    unsafe
    {
        assert_parse_error("[", RegExParserError::BracketMissingClose);
        assert_parse_error("]", RegExParserError::BracketMissingOpen);
        assert_parse_error("[]", RegExParserError::BracketEmpty);
        assert_parse_error("[a][", RegExParserError::BracketMissingClose);
        assert_parse_error("[a]]", RegExParserError::BracketMissingOpen);
        assert_parse_error("[()]", RegExParserError::CharacterMustBeEscaped);

        let to_accept = vec!["a"];
        let to_reject = vec!["", "b", "aa", "aaaa", "aaaaaaa"];
        test_regex("[a]", &to_accept, &to_reject);
        test_regex("([a])", &to_accept, &to_reject);

        let to_accept = vec!["a", "aa", "aaa"];
        let to_reject = vec!["", "b", "c"];
        test_regex("[a]+", &to_accept, &to_reject);

        let to_accept = vec!["", "a", "aa", "aaa"];
        let to_reject = vec!["b", "c"];
        test_regex("[a]*", &to_accept, &to_reject);


        let to_accept = vec!["a", "b"];
        let to_reject = vec!["", "ab", "aa", "bb"];
        test_regex("[ab]", &to_accept, &to_reject);
        test_regex("[a-b]", &to_accept, &to_reject);

        assert_parse_error("[a-]", RegExParserError::DashMissingRHS);
        assert_parse_error("[-a]", RegExParserError::DashMissingLHS);
        assert_parse_error("[-]", RegExParserError::DashMissingLhsAndRhs);
        assert_parse_error("[a--]", RegExParserError::ConsequtiveDashInRange);
        assert_parse_error("[a--b]", RegExParserError::ConsequtiveDashInRange);
        assert_parse_error("[a---]", RegExParserError::ConsequtiveDashInRange);
        assert_parse_error("[a---b]", RegExParserError::ConsequtiveDashInRange);
        assert_parse_error("[a-b-c]", RegExParserError::DashMissingLHS);
        assert_parse_error("[1-0]", RegExParserError::DashRhsIsLowerThanLhs);
        assert_parse_error("[z-a]", RegExParserError::DashRhsIsLowerThanLhs);

        let to_accept = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let to_reject = vec!["", "a", "ab", "10", "11", "02"];
        test_regex("[0-9]", &to_accept, &to_reject);

        let to_accept = vec!["+"];
        let to_reject = vec!["", "a", "/", "\\", " "];
        test_regex("[\\+]", &to_accept, &to_reject);

        let to_accept = vec!["(", ")"];
        let to_reject = vec!["", "a", "/", "\\", " ", "))", "((", "()", ")("];
        test_regex("[\\(-\\)]", &to_accept, &to_reject);
    }
}

#[test]
fn or_tests()
{
    unsafe
    {
        assert_parse_error("|", RegExParserError::OrMissingLhs);
        assert_parse_error("a|", RegExParserError::OrMissingOrInvalidRhs);
        assert_parse_error("|a", RegExParserError::OrMissingLhs);
        assert_parse_error("||", RegExParserError::OrMissingLhs);

        let to_accept = vec!["a", "b"];
        let to_reject = vec!["", "bb", "aa", "aaaa", "cc", " ", "0"];
        test_regex("a|b", &to_accept, &to_reject);

        let to_accept = vec!["a", "b", "aa", "aaaa"];
        let to_reject = vec!["", "bb", "bbb"];
        test_regex("a+|b", &to_accept, &to_reject);

        let to_accept = vec!["a", "b", "bb", "bbbb"];
        let to_reject = vec!["", "aa", "aaa"];
        test_regex("a|b+", &to_accept, &to_reject);
        test_regex("(a)|b+", &to_accept, &to_reject);
        test_regex("[a]|b+", &to_accept, &to_reject);
        test_regex("a|(b+)", &to_accept, &to_reject);
        test_regex("a|(b)+", &to_accept, &to_reject);
        test_regex("a|[b]+", &to_accept, &to_reject);

        let to_accept = vec!["a", "b", "c"];
        let to_reject = vec!["", "bb", "aa", "aaaa", "cc", " ", "0"];
        test_regex("a|b|c", &to_accept, &to_reject);
    }
}