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
    LineEndedPrematurely,
    EmptyLine,
    InvalidGrammarRule,
    MissingGrammarRuleEndSymbol,
    NoGrammarRules,
    InvalidProduction,
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
            FileParserErrorKind::LineEndedPrematurely => "The end of the line while the program expected more characters in the line.",
            FileParserErrorKind::EmptyLine => "The line read was empty but it was not expected to be empty.",
            FileParserErrorKind::InvalidGrammarRule => "The grammar rule is invalid.",
            FileParserErrorKind::MissingGrammarRuleEndSymbol => "The grammar rule is missing the end symbol (;).",
            FileParserErrorKind::NoGrammarRules => "The file has no grammar rules.",
            FileParserErrorKind::InvalidProduction => "The production is not a valid production.",
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

struct GrammarRule {
    pub name: String,
    pub productions: Vec<Vec<String>>,
}

enum FileSection {
    Lexer,
    Grammar,
}

impl FileSection {
    fn as_str(&self) -> &str {
        return match self {
            Self::Lexer => "LEXER",
            Self::Grammar => "GRAMMAR",
        }
    }
}

pub struct FileParser {
    curr_section: FileSection,
}

impl FileParser {
    pub fn new() -> FileParser {
        return FileParser {curr_section: FileSection::Lexer};
    }

    fn is_valid_section_header(&self, line: &str) -> bool {
        if let FileSection::Lexer = self.curr_section {
            return line == "SECTION LEXER";
        }
        else {
            return line == "SECTION GRAMMAR"; 
        }
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

        if !self.is_valid_section_header(line.trim()) {
            return Err(FileParserError::new(FileParserErrorKind::FileDoesNotBeginWithSectionHeader, None));
        }
        self.curr_section = FileSection::Grammar;

        line.clear();

        let mut rules: Vec<Rule> = Vec::new();
        let mut rule_names: HashSet<String> = HashSet::new();
        let mut rule_counter = 1;
        let mut found_grammar_section = false;
        while let Ok(result) = reader.read_line(&mut line) {
            if result == 0 {
                break;
            }

            // Skip empty lines
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if self.is_valid_section_header(&trimmed) {
                found_grammar_section = true;
                break;
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

        // Parse grammar
        if found_grammar_section {
            let result = self.parse_grammar_section(&mut reader);
            //return ;
            if result.is_err() {
                return Err(result.err().unwrap());
            }
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

    fn parse_grammar_section(&self, reader: &mut BufReader<File>) -> Result<Vec<GrammarRule>, FileParserError>
    {
        // Parse each rule until the end
        let mut line = String::new();
        let mut rules: Vec<GrammarRule> = Vec::new();
        let mut prev_rule: Option<GrammarRule> = None;
        let mut in_middle_of_rule = false;

        while let Ok(result) = reader.read_line(&mut line) {
            if result == 0 {
                break;
            }
            
            let chars: Vec<char> = line.chars().collect();

            if in_middle_of_rule { 
                // This will be another production or an end
                // Try to see if the grammar rule ended
                if let Err(err) = self.parse_grammar_rule_end(&chars) {
                    if err.kind != FileParserErrorKind::MissingGrammarRuleEndSymbol {
                        return Err(err);
                    }
                   
                    // Try to read another production
                    let another_prod = self.parse_second_prod(&chars);
                    if let Err(err) = another_prod {
                        if err.kind == FileParserErrorKind::EmptyLine {
                            continue;
                        } else {
                            return Err(err);
                        }
                    }

                    if let Some(rule) = &mut prev_rule {
                        rule.productions.push(another_prod.unwrap());
                    }
                } else {
                    in_middle_of_rule = false;
                    let t = prev_rule.take();
                    rules.push(t.unwrap());
                    //rev_rule = None;
                }
            }
            else {
                // Read first production
                let first_prod = self.parse_first_prod(&chars);
                if let Err(parser_error) = first_prod {
                    if FileParserErrorKind::EmptyLine != parser_error.kind {
                        return Err(parser_error);
                    } 
                } else {
                    prev_rule = Some(first_prod.unwrap());
                    in_middle_of_rule = true;
                }
            }
            line.clear();
        }

        if in_middle_of_rule {
            return Err(FileParserError::new(FileParserErrorKind::MissingGrammarRuleEndSymbol, None));
        }

        if rules.len() == 0 {
            return Err(FileParserError::new(FileParserErrorKind::NoGrammarRules, None))
        }

        return Ok(rules);
    }

    fn parse_first_prod(&self, line: &Vec<char>) -> Result<GrammarRule, FileParserError>
    {
        let name: String;
        let mut line_index: usize = 0;

        // Parse name of the grammar rule
        if let Some(temp_name) = self.parse_identifier(&line, &mut line_index) {
            name = temp_name;
        } else {
            return Err(FileParserError::new(FileParserErrorKind::EmptyLine, None));
        }

        // Check if reached end
        if line_index >= line.len() {
            return Err(FileParserError::new(FileParserErrorKind::LineEndedPrematurely, None));
        }

        // Read colon
        if line[line_index] != ':' {
            return Err(FileParserError::new(FileParserErrorKind::InvalidGrammarRule, None));
        }
        line_index += 1;

        // Read production
        let mut productions: Vec<String> = Vec::new();
        while let Some(temp_name) = self.parse_identifier(&line, &mut line_index) {
            productions.push(temp_name);
        }

        if productions.len() == 0 {
            return Err(FileParserError::new(FileParserErrorKind::InvalidGrammarRule, None));
        }

        let mut rule = GrammarRule { name, productions: Vec::new()};
        rule.productions.push(productions);
        return Ok(rule);
    }

    fn parse_second_prod(&self, line: &Vec<char>) -> Result<Vec<String>, FileParserError>
    {
        let mut line_index: usize = 0;

        // Ensure line starts with |
        if line[line_index] != '|' {
            return Err(FileParserError::new(FileParserErrorKind::InvalidProduction, None));
        }
        line_index += 1;

        // Read production
        let mut productions: Vec<String> = Vec::new();
        while let Some(temp_name) = self.parse_identifier(&line, &mut line_index) {
            productions.push(temp_name);
        }

        if productions.len() == 0 {
            return Err(FileParserError::new(FileParserErrorKind::InvalidProduction, None));
        }
        return Ok(productions);
    }

    fn parse_grammar_rule_end(&self, line: &Vec<char>) -> Result<bool, FileParserError>
    {
        let mut line_index: usize = 0;
        // Ensure line starts with |
        if line[line_index] != ';' {
            return Err(FileParserError::new(FileParserErrorKind::MissingGrammarRuleEndSymbol, None));
        }
        println!("hello2");
        line_index += 1;

        if let Some(temp_name) = self.parse_identifier(&line, &mut line_index) {
            return Err(FileParserError::new(FileParserErrorKind::InvalidGrammarRule, None));
        }
        
        return Ok(true);
    }

    fn parse_identifier(&self, line: &Vec<char>, index: &mut usize) -> Option<String> {
        let mut identifier = String::new();
        let mut skipped_whitespace = false;
        while *index < line.len() {
            if char::is_whitespace(line[*index]) {
                if !skipped_whitespace {
                    *index += 1;
                    continue;
                } else {
                    break;
                }
                
            }
            if line[*index] == ':' {
                break;
            }
            // No longer at whitespace
            skipped_whitespace = true;
            identifier.push(line[*index]);
            *index += 1;
        }
        if identifier.is_empty() {
            return None;
        }
        return Some(identifier);
    }
}