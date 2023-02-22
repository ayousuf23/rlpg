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
        if let Some(base_node) = self.parse_middle() {
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

    fn parse_middle(&mut self) -> Option<Box<Node>> {
        // We want to parse base folloed by one or more +'s

        // First, parse base
        let base = self.parse_base();
        if let None = base {
            return None;
        }
        let base = base.unwrap();

        // Create middle node
        let mut middle = Box::new(Node::new("Middle".to_string(), NodeKind::Middle));

        if (self.current_char == '+' || self.current_char == '*' || self.current_char == '?') && !self.reached_end {

            let node_kind = match self.current_char {
                '+' => NodeKind::MiddlePlus,
                '*' => NodeKind::Star,
                '?' => NodeKind::QuestionMark,
                _ => panic!()
            };

            // Create a '+' node
            let mut op_node = Box::new(Node::new(self.current_char.to_string(), node_kind));
            op_node.add_child(base);
            middle.add_child(op_node);
            
            // Check if base is followed by one or more of the same character
            let curr_char_copy = self.current_char;
            while self.current_char == curr_char_copy && !self.reached_end {
                self.advance();
            }
        } else {
            middle.add_child(base);
        }

        Some(middle)

    }
    
    fn parse_base(&mut self) -> Option<Box<Node>> {
        if self.reached_end {
            return None;
        }

        // Check if the current character is a character
        if self.current_char == '.' {
            let node = Some(Box::new(Node::new(self.current_char.to_string(), NodeKind::BaseAnyChar)));
            self.advance();
            return node;
        }

        // Check for parentheses
        if self.current_char == '(' {
            let mut node = Some(Box::new(Node::new(self.current_char.to_string(), NodeKind::Parentheses)));
            // Parse regex
            if let Some(inner_expression) = self.parse_regex() {
                node.as_mut().unwrap().add_child(inner_expression);
            }
            return node;
        } else if self.current_char == ')' {
            return None;
        }

        if RegExParser::is_non_regex_char(self.current_char) {
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

    fn peek_next_character(&mut self) -> Option<char> {
        return self.iterator.peek().copied();
    }
}