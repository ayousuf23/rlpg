use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

use colored::Colorize;

use crate::NFA;
use crate::grammar2::{Production, Symbol, GrammarRule, Empty};

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
    NoGrammarSection,
    InvalidProduction,
    DuplicateGrammarRuleName,
    UnknownSymbol,
    InvalidIdentifier,
    DuplicateProduction,
    RootRuleDoesNotExist,
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
            FileParserErrorKind::DuplicateGrammarRuleName => "There already exists a grammar rule with the same name.",
            FileParserErrorKind::UnknownSymbol => "This symbol is not defined or has not been defined yet.",
            FileParserErrorKind::InvalidIdentifier => "The identifer is invalid because it contains special or invalid characters.",
            FileParserErrorKind::DuplicateProduction => "The grammar rule contains duplicate productions.",
            FileParserErrorKind::RootRuleDoesNotExist => "A grammar rule with the name of 'root' does not exist.",
            FileParserErrorKind::NoGrammarSection => "The file does not contain a grammar section, which is required.",
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
    symbols: HashMap<String, bool>,
    pub grammar_rules: Vec<GrammarRule>,
    emptiness_info: HashMap<String, Empty>,
    rules: Vec<Rule>,
    undefined_symbols: HashSet<String>,
}

impl FileParser {
    pub fn new() -> FileParser {
        return FileParser {
            curr_section: FileSection::Lexer,
            symbols: HashMap::new(),
            grammar_rules: Vec::new(),
            emptiness_info: HashMap::new(),
            rules: Vec::new(),
            undefined_symbols: HashSet::new(),
        };
    }

    fn is_valid_section_header(&self, line: &str) -> bool {
        if let FileSection::Lexer = self.curr_section {
            return line == "SECTION LEXER";
        }
        else {
            return line == "SECTION GRAMMAR"; 
        }
    }

    pub fn parse_file(&mut self, path: &str) -> Result<(), FileParserError> {
        let file = match File::open(path) {
            Ok(inner_file) => inner_file,
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
                // Return an error if a symbol with the same name already exists
                if let Some(_) = self.symbols.insert(name.to_string(), true) {
                    return Err(FileParserError::new(FileParserErrorKind::DuplicateName, None));
                }
            }

            rules.push(rule);
            line.clear();
        }

        if rule_counter == 1 {
            return Err(FileParserError::new(FileParserErrorKind::NoRules, None));
        }

        if !found_grammar_section {
            return Err(FileParserError { kind: FileParserErrorKind::NoGrammarSection, inner_error: None });
        }

        // Parse grammar
        let result = self.parse_grammar_section(&mut reader);
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        self.grammar_rules = result.unwrap();
        

        self.rules = rules;
        return Ok(());
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

        // Catch invalid names here
        if name == "SECTION" {
            return Err(FileParserError::new(FileParserErrorKind::InvalidRuleName, None));
        }
        else if name == "eof" {
            return Err(FileParserError::new(FileParserErrorKind::InvalidRuleName, None));
        }
        else if name == "root" {
            return Err(FileParserError::new(FileParserErrorKind::InvalidRuleName, None));
        }

        if name == "unnamed" {
            return Ok(RuleKind::Unnamed);
        } else  {
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

    fn parse_grammar_section(&mut self, reader: &mut BufReader<File>) -> Result<Vec<GrammarRule>, FileParserError>
    {
        // Parse each rule until the end
        let mut line = String::new();
        let mut rules: Vec<GrammarRule> = Vec::new();
        let mut prev_rule: Option<GrammarRule> = None;
        let mut in_middle_of_rule = false;
        let mut root_rule_exists = false;

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
                        let another_prod = another_prod.unwrap();
                        rule.productions.push(another_prod);
                    }
                } else {
                    in_middle_of_rule = false;
                    let t = prev_rule.take().unwrap();
                    if self.does_rule_contain_duplicate_prods(&t)
                    {
                        return Err(FileParserError { kind: FileParserErrorKind::DuplicateProduction, inner_error: None });
                    }
                    if t.name == "root" {
                        root_rule_exists = true;
                    }
                    rules.push(t);
                    prev_rule = None;
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
                    let rule = first_prod.unwrap();
                    prev_rule = Some(rule);
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

        // Ensure one rule called root exists
        if !root_rule_exists {
            return Err(FileParserError { kind: FileParserErrorKind::RootRuleDoesNotExist, inner_error: None });
        }

        if self.undefined_symbols.len() > 0 {
            return Err(FileParserError::new(FileParserErrorKind::UnknownSymbol, None));
        }

        return Ok(rules);
    }

    fn parse_first_prod(&mut self, line: &Vec<char>) -> Result<GrammarRule, FileParserError>
    {
        let name: String;
        let mut line_index: usize = 0;

        // Parse name of the grammar rule
        if let Some(temp_name) = self.parse_identifier(&line, &mut line_index) {
            name = temp_name;
        } else {
            return Err(FileParserError::new(FileParserErrorKind::EmptyLine, None));
        }

        if !FileParser::is_identifier_valid(&name)
        {
            return Err(FileParserError::new(FileParserErrorKind::InvalidIdentifier, None));
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

        // Insert into symbols
        if let Some(_) = self.symbols.insert(name.to_string(), false) {
            // Throw a duplicate name error
            return Err(FileParserError::new(FileParserErrorKind::DuplicateGrammarRuleName, None));
        }

        // Read production
        let mut production: Vec<Symbol> = Vec::new();
        while let Some(temp_name) = self.parse_identifier(&line, &mut line_index) {
            // Check if symbol is defined
            if let Some(is_terminal) = self.symbols.get(&temp_name) {
                
                if self.undefined_symbols.contains(&temp_name) {
                    self.undefined_symbols.remove(&temp_name);
                }
                
                // Get emptiness info
                let emptiness = self.get_emptiness_or_default(&temp_name);
                production.push(Symbol { name: temp_name, is_terminal: *is_terminal, emptiness });
            }
            else if !FileParser::is_identifier_valid(&temp_name) {
                return Err(FileParserError::new(FileParserErrorKind::InvalidIdentifier, None));
            }
            else {
                // Throw an error for undefined symbol
                //return Err(FileParserError::new(FileParserErrorKind::UnknownSymbol, None));
                production.push(Symbol { name: temp_name.clone(), is_terminal: false, emptiness: Empty::NonEmpty });
                // Add to undefined list
                self.undefined_symbols.insert(temp_name);
            }
        }

        if production.len() == 0 {
            return Err(FileParserError::new(FileParserErrorKind::InvalidGrammarRule, None));
        }

        let mut rule = GrammarRule { name, productions: Vec::new()};
        rule.productions.push(Box::into_raw(Box::new(Production {prod: production })));
        return Ok(rule);
    }

    fn parse_second_prod(&mut self, line: &Vec<char>) -> Result<*mut Production, FileParserError>
    {
        let mut line_index: usize = 0;

        // Ensure line starts with |
        if line[line_index] != '|' {
            return Err(FileParserError::new(FileParserErrorKind::InvalidProduction, None));
        }
        line_index += 1;

        // Read production
        let mut production: Vec<Symbol> = Vec::new();
        while let Some(temp_name) = self.parse_identifier(&line, &mut line_index) {
            // Check if symbol is defined
            if let Some(is_terminal) = self.symbols.get(&temp_name) {
                if self.undefined_symbols.contains(&temp_name) {
                    self.undefined_symbols.remove(&temp_name);
                }
                let emptiness = self.get_emptiness_or_default(&temp_name);
                production.push(Symbol { name: temp_name, is_terminal: *is_terminal, emptiness });
            }
            else if !FileParser::is_identifier_valid(&temp_name) {
                return Err(FileParserError::new(FileParserErrorKind::InvalidIdentifier, None));
            }
            else {
                // Throw an error for undefined symbol
                //return Err(FileParserError::new(FileParserErrorKind::UnknownSymbol, None));

                production.push(Symbol { name: temp_name.clone(), is_terminal: false, emptiness: Empty::NonEmpty });
                // Add to undefined list
                self.undefined_symbols.insert(temp_name);
            }
        }

        if production.len() == 0 {
            return Err(FileParserError::new(FileParserErrorKind::InvalidProduction, None));
        }
        return Ok(Box::into_raw(Box::new(Production {prod: production})));
    }

    fn parse_grammar_rule_end(&self, line: &Vec<char>) -> Result<bool, FileParserError>
    {
        let mut line_index: usize = 0;
        // Ensure line starts with |
        if line[line_index] != ';' {
            return Err(FileParserError::new(FileParserErrorKind::MissingGrammarRuleEndSymbol, None));
        }
        line_index += 1;

        if let Some(_) = self.parse_identifier(&line, &mut line_index) {
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

    fn does_rule_contain_unknown(&self, rule: &GrammarRule) -> bool
    {
        for prod in &rule.productions {
            unsafe {
                for sym in &(**prod).prod {
                    if !self.symbols.contains_key(&sym.name) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn does_rule_contain_valid_identifiers(&self, rule: &GrammarRule) -> bool
    {
        if !FileParser::is_identifier_valid(&rule.name)
        {
            return false;
        } 

        for prod in &rule.productions {
            unsafe {
                for sym in &(**prod).prod {
                    if !FileParser::is_identifier_valid(&sym.name) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn is_identifier_valid(identifier: &str) -> bool
    {
        if identifier == "eof" {
            return false;
        }

        for c in identifier.chars() {
            if !(('0' <= c && c <= '9') || ('a' <= c && c <= 'z') 
            || ('A' <= c && c <= 'Z') || c == '_' || c == '-') {
                return false;
            }
        }
        return true;
    }

    fn does_rule_contain_duplicate_prods(&self, rule: &GrammarRule) -> bool
    {
        for i in 0..rule.productions.len()
        {
            for j in i+1..rule.productions.len()
            {
                unsafe {
                    if *rule.productions[i] == *rule.productions[j]
                    {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn get_terminals(&self) -> HashSet<Symbol>
    {
        let mut set = HashSet::new();
        for (item, is_terminal) in &self.symbols {
            if *is_terminal {
                set.insert(Symbol {name: item.to_string(), is_terminal: true, emptiness: self.get_emptiness_or_default(&item)});
            }
        }
        return set;
    }

    pub unsafe fn build_nfa(&mut self) -> Result<NFA, Box<dyn Error>>
    {
        let nfa = NFA::build_from_rules(&self.rules);
        if let Err(error) = nfa
        {
            return Err(error);
        }
        let (nfa, symbol_emptiness) = nfa.unwrap();
        // Set emptiness info
        self.emptiness_info = symbol_emptiness;
        for rule in &self.grammar_rules {
            for prod in &rule.productions {
                for sym in &mut (**prod).prod {
                    sym.emptiness = self.get_emptiness_or_default(&sym.name);
                }
            }
        }

        return Ok(nfa);
    }

    pub fn get_emptiness_or_default(&self, name: &str) -> Empty
    {
        if let Some(value) = self.emptiness_info.get(name)
        {
            return value.clone();
        }
        return Empty::NonEmpty;
    }

}