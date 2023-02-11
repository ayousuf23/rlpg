use std::str::Chars;
use std::iter::Peekable;
use crate::node::Node;
use crate::node_kind::NodeKind;

pub struct RegExParser<'a> {
    position: i32,
    pattern: String,
    iterator: Peekable<Chars<'a>>,
    current_char: char,
    reached_end: bool,
}

impl RegExParser<'_> {

    pub fn new(pattern: &str) -> RegExParser {
        let mut parser = RegExParser {
            position: -1,
            pattern: pattern.to_string(),
            iterator: pattern.chars().peekable(),
            current_char: 'a',
            reached_end: false,
            
        };
        parser.advance();
        println!("{}", parser.current_char);
        parser
    }

    pub fn parse(&mut self) -> Box<Node> {
        let mut tree_root = Box::new(Node::new("ROOT".to_string(), NodeKind::Root));
        let regex_node = self.parse_regex().unwrap();
        tree_root.add_child(regex_node);
        return tree_root;
    }

    fn parse_regex(&mut self) -> Option<Box<Node>> {
        // Parse base
        if let Some(base_node) = self.parse_base() {
            // Create a node called regex
            let mut regex_node = Box::new(Node::new("RegEx".to_string(), NodeKind::RegEx));

            // Add node from base to regex
            regex_node.add_child(base_node);
            
            // Parse regex and add node to regex_node
            if let Some(sub_regex) = self.parse_regex() {
                regex_node.add_child(sub_regex);
            }

            return Some(regex_node);
        }
        return None;
    }
    
    fn parse_base(&mut self) -> Option<Box<Node>> {
        // Check if the current character is a character
        if !self.reached_end && RegExParser::is_non_regex_char(self.current_char) {
            let node = Some(Box::new(Node::new(self.current_char.to_string(), NodeKind::Base)));
            // Advance the parser
            self.advance();
            return node;
        }
        return None;
    }

    fn advance(&mut self) {
        if let Some(next) = self.iterator.next() {
            self.current_char = next;
        } else {
            self.reached_end = true;
        }
        self.position += 1;
    }

    fn is_non_regex_char(character: char) -> bool {
        return true;
    }
}