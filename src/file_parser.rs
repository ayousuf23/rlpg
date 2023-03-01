use std::fs::File;
use std::io::{BufReader, BufRead, self};
use std::path::Path;

use crate::error::RlpgErr;

#[derive(Debug, Clone)]
pub enum FileParserErrorKind {
    FileDoesNotBeginWithSectionHeader,
    InvalidActionCode,
    InvalidRuleName,
    InvalidRegex,
}

#[derive(Debug, Clone)]
pub struct FileParserError {
    pub kind: FileParserErrorKind,
}

impl FileParserError {
    pub fn new(kind: FileParserErrorKind) -> FileParserError {
        return FileParserError { kind };
    }
}

impl RlpgErr for FileParserError {
    pub fn get_err_message(&self) -> String {
        return match self.kind {
            FileParserErrorKind::FileDoesNotBeginWithSectionHeader => "The input file does not begin with a section header",
            FileParserErrorKind::InvalidActionCode => "The action code is invalid",
            FileParserErrorKind::InvalidRegex => "The regex is invalid",
            FileParserErrorKind::InvalidRuleName => "The rule name is invalid",
        }
    }
}

#[derive(Debug)]
pub enum RuleKind {
    Named(String),
    Unnamed,
}

#[derive(Debug)]
pub struct Rule {
    pub kind: RuleKind,
    pub regex: String,
    pub action: Option<String>,
    pub priority: i32,
}

pub struct FileParser {
    pub rules: Vec<Rule>
}

impl FileParser {
    fn is_valid_section_header(line: &str) -> bool {
        return line == "SECTION LEXER";
    }

    pub fn parse_file(&mut self, path: &str) -> Result<bool, FileParserError> {
        let file = File::open(path).expect("Error file cound not be opened!");

        let mut reader = BufReader::new(file);

        let mut line: String = String::new();
        reader.read_line(&mut line).expect("Error");

        if !FileParser::is_valid_section_header(line.trim()) {
            return Err(FileParserError::new(FileParserErrorKind::FileDoesNotBeginWithSectionHeader));
        }

        line.clear();

        let mut rule_counter = 1;
        while let Ok(result) = reader.read_line(&mut line) {
            if result == 0 {
                break;
            }

            // Skip empty lines
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let rule = FileParser::parse_rule(&line);
            if rule.is_err() {
                return Err(rule.unwrap_err());
            }
            let mut rule = rule.unwrap();
            rule.priority = rule_counter;
            rule_counter += 1;
            self.rules.push(rule);
            line.clear();
        }

        return Ok(true);
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn parse_rule(line: &str) -> Result<Rule, FileParserError> {
        let parts = FileParser::parse_line(line);
        //println!("{:?}", parts);

        // Get kind
        let kind = FileParser::determine_rule_kind(parts[0].to_string());
        if kind.is_err() {
            return Err(kind.unwrap_err());
        }

        // Get regex
        let regex = FileParser::validate_regex(parts[1].to_string());
        if regex.is_err() {
            return Err(regex.unwrap_err());
        }

        // Get action code
        let action_code = FileParser::get_action_code(parts[2].to_string());
        if action_code.is_err() {
            return Err(action_code.unwrap_err());
        }

        return Ok(Rule {kind: kind.unwrap(), regex: regex.unwrap(), action: action_code.unwrap(), priority: 1});
    }

    fn get_action_code(code: String) -> Result<Option<String>, FileParserError> {
        if code.is_empty() {
            return Ok(None);
        }
        if !code.starts_with('{') || !code.ends_with('}') {
            // Throw an error
            return Err(FileParserError::new(FileParserErrorKind::InvalidActionCode));
        }

        return Ok(Some(code));
    }

    fn validate_regex(regex: String) -> Result<String, FileParserError> {
        if regex.is_empty() {
            return Err(FileParserError::new(FileParserErrorKind::InvalidRegex));
        }
        return Ok(regex);
    }

    fn determine_rule_kind(name: String) -> Result<RuleKind, FileParserError> {
        if name.is_empty() {
            return Err(FileParserError::new(FileParserErrorKind::InvalidRuleName));
        }

        if name == "unnamed" {
            return Ok(RuleKind::Unnamed);
        } else {
            return Ok(RuleKind::Named(name));
        }
    }

    fn parse_line(line: &str) -> [String; 3] {
        let mut escaped = false;

        let mut parts: [String; 3] = std::array::from_fn(|_i| String::new());

        let mut curr_i = 0;

        for c in line.chars() {
            if c.is_whitespace() && !(c == ' ' && escaped) {  
                if parts[curr_i].len() > 0 {
                    if curr_i == 2 {
                        break;
                    }
                    curr_i += 1;
                }
                continue;
            }

            if c == '\\' && !escaped {
                escaped = true;
            } else if escaped {
                escaped = false;
            }

            
            parts[curr_i].push(c);
        }
        return parts;
    }
}