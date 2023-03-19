use std::fmt::Result;
use std::str::Chars;
use std::iter::Peekable;
use crate::node::Node;
use crate::node_kind::NodeKind;

pub enum RegExParserError
{
    ParseError,
}

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
        parser
    }

    pub fn parse(&mut self) -> Result<Box<Node>, RegExParserError>  {
        let mut tree_root = Box::new(Node::new("ROOT".to_string(), NodeKind::Root));
        if let Some(regex_node) = self.parse_regex()
        {
            tree_root.add_child(regex_node);
            return tree_root;
        }
        else
        {
           // Return a result object...
           return RegExParserError::ParseError;
        }
    }

    fn parse_regex(&mut self) -> Result {
        // Parse base
        let result = self.parse_high();
        if result.is_err()
        {
            return result;
        }
        let base_node = result?;

        // Create a node called regex
        let mut regex_node = Box::new(Node::new("RegEx".to_string(), NodeKind::RegEx));

        // Add node from base to regex
        regex_node.add_child(base_node);
        
        // Parse regex and add node to regex_node
        if let Ok(sub_regex) = self.parse_regex() {
            regex_node.add_child(sub_regex);
        }

        return Ok(regex_node);
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

    fn parse_high(&mut self) -> Result<Box<Node>, RegExParserError> {
        // high = middle | middle
        // high = middle
        if let Some(middle1) = self.parse_middle() {
            if self.current_char == '|' && !self.reached_end {
                self.advance();
                if let Some(middle2) = self.parse_middle() {
                    // Create a new start node
                    let mut node = Box::new(Node::new("high".to_string(), NodeKind::High));
                    node.add_child(middle1);
                    node.add_child(middle2);
                    return Some(node);
                } else {
                    panic!("The OR operator | is not followed by an expresion.");
                }
            } else {
                return Some(middle1);
            }
        }
        None
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

        let char = self.parse_valid_or_escaped_char_as_char();
        return Some(Box::new(Node::new(char.to_string(), NodeKind::Base)));
    }

    fn parse_bracket(&mut self) -> Option<Box<Node>> {
        // Accepts a character or a range
        let mut inner_bracket_node = Node::new("[".to_string(), NodeKind::Bracket);

        //let mut prev_char: Option<char> = None;

        let mut range_started = false;

        let mut lhs: Option<char> = None;

        while self.current_char != ']' && !self.reached_end {
            // Create a node for each character
            

            if self.current_char == '-' {
                if range_started {
                    panic!("Two or more '-''s are not allowed after each other in a range.");
                }   
                range_started = true;
                self.advance();
                continue;
            }
            
            // Get the valid or escaped character
            let cur_char = self.parse_valid_or_escaped_char_as_char();

            
            if range_started {
                if let Some(real_lhs) = lhs {
                    if !RegExParser::validate_range(real_lhs, cur_char) {
                        panic!("This range is not valid because the character on the left-hand side must be equal to or lower than the character on the right-hand side in value.")
                    }
                    let end_char = char::from_u32(cur_char as u32 + 1).unwrap();
                    for i in real_lhs..end_char {
                        let node = Box::new(Node::new(i.to_string(), NodeKind::Base));
                        inner_bracket_node.add_child(node);
                    }
                    range_started = false;
                    lhs = None;
                } else {
                    panic!("Range is not valid because it is missing a character to the left of the dash.");
                }
            } else {
                if lhs.is_some() {
                    let node = Box::new(Node::new(lhs.unwrap().to_string(), NodeKind::Base));
                    inner_bracket_node.add_child(node);
                }

                lhs = Some(cur_char);
            }

            // Peek to see if the next character is a -
            /*if let Some(next_char) = self.peek_next_character() {
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

            if range_started {
                panic!();
            }

            self.advance();*/
        }

        if range_started {
            panic!();
        }

        if lhs.is_some() {
            let node = Box::new(Node::new(lhs.unwrap().to_string(), NodeKind::Base));
            inner_bracket_node.add_child(node);
            lhs = None;
        }

        if self.current_char == ']' {
            self.advance();
        } else {
            panic!("Bracket does not have a closing ']'");
        }

        return Some(Box::new(inner_bracket_node));
    }

    fn parse_valid_or_escaped_char(&mut self) -> Box<Node>
    {
        let mut to_return;
        if self.current_char == '\\' {
            if let Some(next) = self.peek_next_character() {
                to_return = Box::new(Node::new(self.current_char.to_string(), NodeKind::Base));
                self.advance();
            } else {
                panic!("Error: escape symbol is not followed by a character to escape");
            }
        } else {
            if RegExParser::does_char_require_escape(self.current_char) {
                panic!("Error: this character must be escaped before using it literally");
            }
            to_return = Box::new(Node::new(self.current_char.to_string(), NodeKind::Base));
        }
        self.advance();
        return to_return;
    }

    fn parse_valid_or_escaped_char_as_char(&mut self) -> char
    {
        let mut to_return;
        if self.current_char == '\\' {
            if let Some(next) = self.peek_next_character() {
                to_return = next;
                self.advance();
            } else {
                panic!("Error: escape symbol is not followed by a character to escape");
            }
        } else {
            if RegExParser::does_char_require_escape(self.current_char) {
                panic!("Error: this character must be escaped before using it literally");
            }
            to_return = self.current_char;
        }
        self.advance();
        return to_return;
    }

    fn advance(&mut self) {
        if let Some(next) = self.iterator.next() {
            self.current_char = next;
        } else {
            self.reached_end = true;
        }
        self.position += 1;
    }

    fn does_char_require_escape(character: char) -> bool {
        return match character {
            '+' | '*' | '?' | '-' | '(' | ')' | '.' | '[' | ']' | '|' | '\\' | '"' => true,
            _ => false
        }
    }

    fn peek_next_character(&mut self) -> Option<char> {
        return self.iterator.peek().copied();
    }

    fn validate_range(lower: char, higher: char) -> bool {
        return lower <= higher;
    }
}