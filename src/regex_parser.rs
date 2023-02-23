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
    open_parenthesis_cnt: i32,
    close_parenthesis_cnt: i32,
}

impl RegExParser<'_> {

    pub fn new(pattern: &str) -> RegExParser {
        let mut parser = RegExParser {
            position: -1,
            pattern: pattern.to_string(),
            iterator: pattern.chars().peekable(),
            current_char: 'a',
            reached_end: false,
            open_parenthesis_cnt: 0,
            close_parenthesis_cnt: 0,
            
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

    fn parse_high(&mut self) -> Option<Box<Node>> {
        
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
            self.open_parenthesis_cnt += 1;
            let mut node = Some(Box::new(Node::new(self.current_char.to_string(), NodeKind::Parentheses)));
            self.advance();
            // Parse regex
            if let Some(inner_expression) = self.parse_regex() {
                node.as_mut().unwrap().add_child(inner_expression);
            }
            return node;
        } else if self.current_char == ')' {
            if self.open_parenthesis_cnt - self.close_parenthesis_cnt == 1 {
                self.close_parenthesis_cnt += 1;
                self.advance();
                return None;
            } else {
                // Throw an error
                panic!("Invalid regular expression! Number of closed parentheses encountered is not equal
                to the number of open parenthesis encountered");
            }
            
        }

        if self.current_char == '[' {
            self.advance();
            return self.parse_bracket();
        }

        if RegExParser::is_non_regex_char(self.current_char) {
            let node = Some(Box::new(Node::new(self.current_char.to_string(), NodeKind::Base)));
            // Advance the parser
            self.advance();
            return node;
        }
        return None;
    }

    fn parse_bracket(&mut self) -> Option<Box<Node>> {
        // Accepts a character or a range
        let mut inner_bracket_node = Node::new("[".to_string(), NodeKind::Bracket);

        let mut prev_char = None;

        let mut range_started = false;

        while self.current_char != ']' && !self.reached_end {
            // Create a node for each character

            // Peek to see if the next character is a -
            if let Some(next_char) = self.peek_next_character() {
                if next_char == '-' {
                    if range_started {
                        // Throw an error: two -'s after each other
                        panic!("Two or more '-''s are not allowed after each other in a range.");
                    }
                    prev_char = Some(self.current_char);
                    range_started = true;
                } else if range_started {
                    // Create a range between left to right
                    if !RegExParser::validate_range(prev_char.unwrap(), next_char) {
                        panic!("This range is not valid because the character on the left-hand side must be equal to or lower than the character on the right-hand side in value.")
                    }
                    let end_char = char::from_u32(next_char as u32 + 1).unwrap();
                    for i in prev_char.unwrap()..end_char {
                        let node = Box::new(Node::new(i.to_string(), NodeKind::Base));
                        inner_bracket_node.add_child(node);
                    }
                    range_started = false;
                    self.advance();
                }
            } else if range_started {
                // Throw an error: range has no more characters
                panic!("Range is not valid because it is missing a character to the right of the dash.");
            }

            if self.current_char == '-' {
                panic!("Range is not valid because it is missing a character to the left of the dash.");
            }

            if !range_started {
                let child_node = Box::new(Node::new(self.current_char.to_string(), NodeKind::Base));
                inner_bracket_node.add_child(child_node);
            }
            self.advance();
        }

        if self.current_char == ']' {
            self.advance();
        } else {
            panic!("Bracket does not have a closing ']'");
        }

        return Some(Box::new(inner_bracket_node));
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

    fn validate_range(lower: char, higher: char) -> bool {
        return lower <= higher;
    }
}