use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

use colored::Colorize;

#[derive(Debug, PartialEq)]
pub enum FileParserErrorKind {
    FileDoesNotBeginWithSectionHeader,
    InvalidActionCode,
    InvalidRuleName,
    InvalidRegex,
    NoRules,
    DuplicateName,
    ReadLineError,
    FileOpenError,
}

#[derive(Debug)]
pub struct FileParserError {
    pub kind: FileParserErrorKind,
    pub inner_error: Option<Box<dyn Error>>,
}

impl FileParserError {
    pub fn new(kind: FileParserErrorKind, error: Option<Box<dyn Error>>) -> FileParserError {
        return FileParserError { kind, inner_error: error };
    }

    fn get_err_message(&self) -> String {
        let msg = match self.kind {
            FileParserErrorKind::FileDoesNotBeginWithSectionHeader => "The input file does not begin with a section header",
            FileParserErrorKind::InvalidActionCode => "The action code is invalid",
            FileParserErrorKind::InvalidRegex => "The regex is invalid",
            FileParserErrorKind::InvalidRuleName => "The rule name is invalid",
            FileParserErrorKind::NoRules => "The file has no rules.",
            FileParserErrorKind::DuplicateName => "There are at least two named rules with the same name.",
            FileParserErrorKind::ReadLineError => "An error was encountered while reading a line in the file.",
            FileParserErrorKind::FileOpenError => "An error was encountered while opening the file.",

        };
        return msg.to_string();
    }
}

impl std::fmt::Display for FileParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!("Error: {}", self.get_err_message()).red();
        let result = writeln!(f, "{}", msg);
        
        if let Some(error) = &self.inner_error {
            let msg = format!("{}", error.as_ref()).red();
            writeln!(f, "{}", msg);
        }
        return result;
    }
}

impl std::error::Error for FileParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
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
}

impl FileParser {
    fn is_valid_section_header(line: &str) -> bool {
        return line == "SECTION LEXER";
    }

    pub fn parse_file(&mut self, path: &str) -> Result<Vec<Rule>, FileParserError> {
        let file = match File::open(path) {
            Ok(innerFile) => innerFile,
            Err(error) => return Err(FileParserError::new(FileParserErrorKind::FileOpenError, Some(Box::new(error)))),
        };

        let mut reader = BufReader::new(file);

        let mut line: String = String::new();
        if let Err(error) = reader.read_line(&mut line) {
            return Err(FileParserError::new(FileParserErrorKind::ReadLineError, Some(Box::new(error))));
        }

        if !FileParser::is_valid_section_header(line.trim()) {
            return Err(FileParserError::new(FileParserErrorKind::FileDoesNotBeginWithSectionHeader, None));
        }

        line.clear();

        let mut rules: Vec<Rule> = Vec::new();
        let mut rule_names: HashSet<String> = HashSet::new();
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

            if let RuleKind::Named(name) = &rule.kind {
                if !rule_names.insert(name.to_string()) {
                    return Err(FileParserError::new(FileParserErrorKind::DuplicateName, None));
                }
            }

            rules.push(rule);
            line.clear();
        }

        if rule_counter == 1 {
            return Err(FileParserError::new(FileParserErrorKind::NoRules, None));
        }

        return Ok(rules);
    }

    fn parse_rule(line: &str) -> Result<Rule, FileParserError> {
        let parts = FileParser::parse_line(line);

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
        let action_code = FileParser::get_action_code(parts[2].trim().to_string());
        if action_code.is_err() {
            return Err(action_code.unwrap_err());
        }

        return Ok(Rule {kind: kind.unwrap(), regex: regex.unwrap(), action: action_code.unwrap(), priority: 1});
    }

    fn get_action_code(code: String) -> Result<Option<String>, FileParserError> {
        if code.is_empty() {
            return Ok(None);
        }
        //println!("{}", code);
        if !code.starts_with('{') || !code.ends_with('}') {
            // Throw an error
            return Err(FileParserError::new(FileParserErrorKind::InvalidActionCode, None));
        }

        return Ok(Some(code));
    }

    fn validate_regex(regex: String) -> Result<String, FileParserError> {
        if regex.is_empty() {
            return Err(FileParserError::new(FileParserErrorKind::InvalidRegex, None));
        }
        return Ok(regex);
    }

    fn determine_rule_kind(name: String) -> Result<RuleKind, FileParserError> {
        if name.is_empty() {
            return Err(FileParserError::new(FileParserErrorKind::InvalidRuleName, None));
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
                if parts[curr_i].len() > 0 && curr_i < 2 {
                    curr_i += 1;
                }
                if curr_i < 2 {
                    continue;
                }
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