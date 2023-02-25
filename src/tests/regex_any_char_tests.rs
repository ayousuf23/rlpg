#[cfg(test)]
mod tests {
    use crate::{NFABuilder, regex_parser::RegExParser, NFA};

    fn get_nfa(regex: &str) -> NFA {
        let regex = regex.trim().to_string();
        // Create a regex parser
        let mut parser = RegExParser::new(&regex);
        let parse_root = parser.parse();

        // Generate an NFA
        return NFABuilder::build(&parse_root).expect("Error");
    }

    #[test]
    pub fn test_does_not_accept_empty_string() {
        let nfa = get_nfa(".");
        assert_eq!(false, nfa.simulate(""));
    }

    #[test]
    pub fn test_any_char_works() {
        let nfa = get_nfa("a.");
        assert_eq!(true, nfa.simulate("aa"));
        assert_eq!(true, nfa.simulate("ab"));
        assert_eq!(true, nfa.simulate("aw"));
        assert_eq!(true, nfa.simulate("aq"));
        assert_eq!(true, nfa.simulate("al"));
    }

    #[test]
    pub fn test_any_char_mark_does_not_accept_non_matching_strings() {
        let nfa = get_nfa("a.");
        assert_eq!(false, nfa.simulate("b"));
        assert_eq!(false, nfa.simulate("bc"));
        assert_eq!(false, nfa.simulate("abcd78a"));
        assert_eq!(false, nfa.simulate("aaaaaaaab"));
        assert_eq!(false, nfa.simulate("a"));
        assert_eq!(false, nfa.simulate("aaaaaaaaa"));
    }

    #[test]
    pub fn test_multiple_any_char_works() {
        let nfa = get_nfa("....");
        assert_eq!(true, nfa.simulate("aaaa"));
        assert_eq!(true, nfa.simulate("bbbb"));
        assert_eq!(true, nfa.simulate("hd23"));
        
        assert_eq!(false, nfa.simulate("abcd78a"));
        assert_eq!(false, nfa.simulate("aaaaaaaab"));

        assert_eq!(false, nfa.simulate(""));
    }

}