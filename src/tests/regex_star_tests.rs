#[cfg(test)]
mod tests {
    use crate::{nfa::NFA, nfa_builder::NFABuilder, regex_parser::RegExParser};


    fn get_nfa(regex: &str) -> NFA {
        let regex = regex.trim().to_string();
        // Create a regex parser
        let mut parser = RegExParser::new(&regex);
        let mut parse_root = parser.parse();

        // Generate an NFA
        return NFABuilder::build(&parse_root).expect("Error");
    }

    #[test]
    pub fn test_does_accept_empty_string() {
        let nfa = get_nfa("a*");
        assert_eq!(true, nfa.simulate(""));
    }

    #[test]
    pub fn test_star_works() {
        let nfa = get_nfa("a*");
        assert_eq!(true, nfa.simulate("a"));
        assert_eq!(true, nfa.simulate("aa"));
        assert_eq!(true, nfa.simulate("aaaaaaaaa"));
    }

    #[test]
    pub fn test_star_does_not_accept_not_matching_strings() {
        let nfa = get_nfa("a*");
        assert_eq!(false, nfa.simulate("b"));
        assert_eq!(false, nfa.simulate("bc"));
        assert_eq!(false, nfa.simulate("abcd78a"));
        assert_eq!(false, nfa.simulate("aaaaaaaab"));
    }

    #[test]
    pub fn test_multiple_star_works() {
        let nfa = get_nfa("a******");
        assert_eq!(true, nfa.simulate("a"));
        assert_eq!(true, nfa.simulate("aa"));
        assert_eq!(true, nfa.simulate("aaaaaaa"));
        assert_eq!(true, nfa.simulate(""));
        assert_eq!(false, nfa.simulate("abcd78a"));
        assert_eq!(false, nfa.simulate("aaaaaaaab"));
    }

}