use std::fs::File;
use std::io::{BufReader, BufRead, self};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum FileParserErrorKind {
    FileDoesNotBeginWithSectionHeader,
    InvalidActionCode,
    InvalidRuleName,
    InvalidRegex,
}

#[derive(Debug, Clone)]
pub struct FileParserError {
    kind: FileParserErrorKind,
}

impl FileParserError {
    pub fn new(kind: FileParserErrorKind) -> FileParserError {
        return FileParserError { kind };
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

        while let Ok(result) = reader.read_line(&mut line) {
            if result == 0 {
                break;
            }

            // Skip empty lines
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            for item in FileParser::parse_line(trimmed) {
                
                // Convert the line to a rule
                let rule = FileParser::parse_rule(&item);
                if rule.is_err() {
                    return Err(rule.unwrap_err());
                }
                self.rules.push(rule.unwrap());
            }

            
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

        return Ok(Rule {kind: kind.unwrap(), regex: regex.unwrap(), action: action_code.unwrap()});
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

        let mut curr_part = &mut parts[curr_i];

        for c in line.chars() {
            if c.is_whitespace() && !(c == ' ' && escaped) {     
                if curr_part.len() > 0 && curr_i < 2 {
                    // Go to next part
                    curr_i += 1;
                    curr_part = &mut parts[curr_i];
                }
                continue;
            }

            if c == '\\' && !escaped {
                escaped = true;
            } else if escaped {
                escaped = false;
            }

            curr_part.push(c);
        }
        return parts;
    }
}