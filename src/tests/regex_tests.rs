use crate::{NFABuilder, regex_parser::RegExParser, NFA, DFABuilder, dfa_simulator::DFASimulator};

unsafe fn test_parse(regex: &str, expected_result: bool)
{
    let mut parser = RegExParser::new(regex);
    let parse_root = parser.parse();
    assert!(expected_result == !parse_root.is_err())
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
        test_parse("", false);
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
        test_parse("()", false);
        test_parse("(", false);
        test_parse(")", false);
        test_parse("a(", false);
        test_parse("a)", false);
        test_parse("a()", false);

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
        let to_accept = vec!["", "a", "aa", "aaa", "aaaaaaaaaaa"];
        let to_reject = vec!["b", "aaaaaaaab", "a."];
        test_regex("a*", &to_accept, &to_reject);

        test_regex("a****", &to_accept, &to_reject);
    }
}