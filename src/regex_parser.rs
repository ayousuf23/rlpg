use std::error::Error;
use std::fmt::Display;
use std::str::Chars;
use std::iter::Peekable;
use crate::node::Node;
use crate::node_kind::NodeKind;

#[derive(Debug, PartialEq)]
pub enum RegExParserError
{
    UnmatchedOpenAndCloseParentheses,
    UnexpectedCharacter,
    OrMissingRHS,
    RechedEnd,
    ConsequtiveDashInRange,
    DashMissingLHS,
    DashRhsIsLowerThanLhs,
    BracketMissingClose,
    CharacterMustBeEscaped,
    EscapeNotFollowedByCharacter,
    CloseParenthesisPropogation, // This is not really an error 
    InvalidInnerParenthesesExpression,
}

impl Display for RegExParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for RegExParserError {
    fn description(&self) -> &str
    {
        match self {
            Self::OrMissingRHS => "The OR operator | is not followed by an expresion.",
            Self::UnmatchedOpenAndCloseParentheses => "Invalid regular expression! Number of closed parentheses encountered is not equal
            to the number of open parenthesis encountered",
            Self::ConsequtiveDashInRange => "Two or more '-''s are not allowed after each other in a range.",
            Self::DashRhsIsLowerThanLhs => "This range is not valid because the character on the left-hand side must be equal to or lower than the character on the right-hand side in value.",
            Self::DashMissingLHS => "Range is not valid because it is missing an expression to the left of the dash.",
            Self::BracketMissingClose => "Bracket does not have a closing ']'",
            Self::EscapeNotFollowedByCharacter => "Escape symbol is not followed by a character to escape",
            Self::CharacterMustBeEscaped => "This character must be escaped before using it literally",
            Self::UnexpectedCharacter => "This character was not expected to occur in this position",
            Self::InvalidInnerParenthesesExpression => "The inner parentheses expression is invalid or non-existant.",
            Self::CloseParenthesisPropogation => "This is not an error.",
            Self::RechedEnd => "This is not an error. Reached end of string."
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub struct RegExParser<'a> {
    position: i32,
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
            iterator: pattern.chars().peekable(),
            current_char: 'a',
            reached_end: false,
            open_parenthesis_cnt: 0,
            close_parenthesis_cnt: 0,
            
        };
        parser.advance();
        parser
    }

    pub fn parse(&mut self) -> std::result::Result<Box<Node>, RegExParserError>  {
        let mut tree_root = Box::new(Node::new("ROOT".to_string(), NodeKind::Root));
        let regex_node = self.parse_regex();
        if regex_node.is_err() {
            return regex_node;
        }

        if self.open_parenthesis_cnt != self.close_parenthesis_cnt
        {
            return Err(RegExParserError::UnmatchedOpenAndCloseParentheses);
        }
        tree_root.add_child(regex_node?);
        return Ok(tree_root);
    }

    fn parse_regex(&mut self) -> std::result::Result<Box<Node>, RegExParserError> {
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
        let sub_regex = self.parse_regex();

        if sub_regex.is_ok() {
            regex_node.add_child(sub_regex?);
        } else {
            let err = sub_regex.err().unwrap();
            if err != RegExParserError::RechedEnd && err != RegExParserError::CloseParenthesisPropogation {
                return Err(err);
            }
        }

        return Ok(regex_node);
    }

    fn parse_middle(&mut self) -> std::result::Result<Box<Node>, RegExParserError> {
        // We want to parse base folloed by one or more +'s

        // First, parse base
        let base = self.parse_base();
        if base.is_err()
        {
            return base;
        }

        let base = base?;

        // Create middle node
        let mut middle = Box::new(Node::new("Middle".to_string(), NodeKind::Middle));

        if (self.current_char == '+' || self.current_char == '*' || self.current_char == '?') && !self.reached_end {

            let node_kind = match self.current_char {
                '+' => NodeKind::MiddlePlus,
                '*' => NodeKind::Star,
                '?' => NodeKind::QuestionMark,
                _ => {
                    return Err(RegExParserError::UnexpectedCharacter);
                }
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

        Ok(middle)
    }

    fn parse_high(&mut self) -> std::result::Result<Box<Node>, RegExParserError> {
        // high = middle | middle
        // high = middle
        let middle1 = self.parse_middle();
        if middle1.is_err()
        {
            return middle1;
        }
        let middle1 = middle1?;

        if self.current_char == '|' && !self.reached_end {
            self.advance();
            if let Ok(middle2) = self.parse_middle() {
                // Create a new start node
                let mut node = Box::new(Node::new("high".to_string(), NodeKind::High));
                node.add_child(middle1);
                node.add_child(middle2);
                return Ok(node);
            } else {
                return Err(RegExParserError::OrMissingRHS);
            }
        } else {
            return Ok(middle1);
        }
    }
    
    fn parse_base(&mut self) -> std::result::Result<Box<Node>, RegExParserError> {
        if self.reached_end {
            return Err(RegExParserError::RechedEnd);
        }

        // Check if the current character is a character
        if self.current_char == '.' {
            let node = Box::new(Node::new(self.current_char.to_string(), NodeKind::BaseAnyChar));
            self.advance();
            return Ok(node);
        }

        // Check for parentheses
        if self.current_char == '(' {
            self.open_parenthesis_cnt += 1;
            let mut node = Box::new(Node::new(self.current_char.to_string(), NodeKind::Parentheses));
            self.advance();
            // Parse regex
            if let Ok(inner_expression) = self.parse_regex() {
                node.as_mut().add_child(inner_expression);
                return Ok(node);
            }
            return Err(RegExParserError::InvalidInnerParenthesesExpression);
        } else if self.current_char == ')' {
            self.close_parenthesis_cnt += 1;
            self.advance();
            return Err(RegExParserError::CloseParenthesisPropogation);     
        }

        if self.current_char == '[' {
            self.advance();
            return self.parse_bracket();
        }

        return match self.parse_valid_or_escaped_char_as_char() {
            Ok(char) => Ok(Box::new(Node::new(char.to_string(), NodeKind::Base))),
            Err(err) => Err(err),
        };
    }

    fn parse_bracket(&mut self) -> std::result::Result<Box<Node>, RegExParserError> {
        // Accepts a character or a range
        let mut inner_bracket_node = Node::new("[".to_string(), NodeKind::Bracket);

        let mut range_started = false;

        let mut lhs: Option<char> = None;

        while self.current_char != ']' && !self.reached_end {
            // Create a node for each character

            if self.current_char == '-' {
                if range_started {
                    return Err(RegExParserError::ConsequtiveDashInRange);
                }   
                range_started = true;
                self.advance();
                continue;
            }
            
            // Get the valid or escaped character
            let cur_char = match self.parse_valid_or_escaped_char_as_char() {
                Ok(char) => char,
                Err(err) => {
                    return Err(err);
                }
            };
            
            if range_started {
                if let Some(real_lhs) = lhs {
                    if !RegExParser::validate_range(real_lhs, cur_char) {
                        return Err(RegExParserError::DashRhsIsLowerThanLhs);
                    }
                    let end_char = char::from_u32(cur_char as u32 + 1).unwrap();
                    for i in real_lhs..end_char {
                        let node = Box::new(Node::new(i.to_string(), NodeKind::Base));
                        inner_bracket_node.add_child(node);
                    }
                    range_started = false;
                    lhs = None;
                } else {
                    return Err(RegExParserError::DashMissingLHS);
                }
            } else {
                if lhs.is_some() {
                    let node = Box::new(Node::new(lhs.unwrap().to_string(), NodeKind::Base));
                    inner_bracket_node.add_child(node);
                }

                lhs = Some(cur_char);
            }
        }

        if range_started {
            return Err(RegExParserError::DashMissingLHS);
        }

        if lhs.is_some() {
            let node = Box::new(Node::new(lhs.unwrap().to_string(), NodeKind::Base));
            inner_bracket_node.add_child(node);
            lhs = None;
        }

        if self.current_char == ']' {
            self.advance();
        } else {
            return Err(RegExParserError::BracketMissingClose);
        }

        return Ok(Box::new(inner_bracket_node));
    }

    fn parse_valid_or_escaped_char_as_char(&mut self) -> Result<char, RegExParserError>
    {
        let to_return;
        if self.current_char == '\\' {
            if let Some(next) = self.peek_next_character() {
                to_return = next;
                self.advance();
            } else {
                return Err(RegExParserError::EscapeNotFollowedByCharacter);
            }
        } else {
            if RegExParser::does_char_require_escape(self.current_char) {
                return Err(RegExParserError::CharacterMustBeEscaped);
            }
            to_return = self.current_char;
        }
        self.advance();
        return Ok(to_return);
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