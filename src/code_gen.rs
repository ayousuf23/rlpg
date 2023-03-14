use std::{any, io::Write};

use crate::{table_dfa_builder::TableDFA, nfa::TransitionKind, token::Token};
use std::fs::File;

pub struct CodeGen
{
    pub table: TableDFA,
    pub curr_state_name: String,
}

impl CodeGen {

    fn write_to_file(path: String, text: String)
    {
        let mut file = File::create(path)?;
        file.write_all(text.as_bytes());
    }

    pub fn generate_lexer(&mut self) -> String
    {
        let mut text = self.create_transition_kind();
        text += "\n";
        text += &self.create_check_accepting_state_function();
        text += "\n";
        text += &self.create_transition_function();
        text += "\n";
        text += &self.create_parse_function();
        return text;
    }

    /*pub fn parse(text: String) -> Vec<String>
    {
        let mut curr_state = 0;
        let seq: Vec<char> = text.chars().collect();
        let mut index = 0;
        let mut tokens: Vec<String> = Vec::new();

        while index < seq.len() {
            // Check if accepting state
            if let Some(token) = is_accepting(curr_state)
            {
                tokens.push(token);
            }

            // Perform transition or error
            let trans_kind = TransitionKind::Character(seq[index]);
            curr_state = transition(curr_state, trans_kind);
            index += 1;
        }

        tokens
    }*/

    pub fn create_parse_function(&mut self) -> String
    {
        "pub fn parse(text: String) -> Vec<String> 
        {
            let mut curr_state = 1;
            let seq: Vec<char> = text.chars().collect();
            let mut index = 0;
            let mut tokens: Vec<String> = Vec::new();
            while index < seq.len() {
                // Check if accepting state
                if let Some(token) = is_accepting(curr_state)
                {
                    tokens.push(token);
                }
                
                // Perform transition or error
                let trans_kind = TransitionKind::Character(seq[index]);
                curr_state = transition(curr_state, trans_kind);
                index += 1;
            }
            if let Some(token) = is_accepting(curr_state)
            {
                tokens.push(token);
            }
            tokens
        }".to_string()
    }

    pub fn create_transition_kind(&mut self) -> String 
    {
        return "pub enum TransitionKind {\n\
            \tCharacter(char),\n
            \tAnyChar,\n
        }".to_string();
    }

    pub fn create_check_accepting_state_function(&mut self) -> String
    {
        let mut header: String = "fn is_accepting(state: i32) -> Option<String>\n{\n".to_string();
        let mut match_statement: String = "\treturn match state {\n".to_string();
        for state in &self.table.accepting_states
        {
            match_statement += &format!("\t\t{0} => Some(\"{1}\".to_string()),\n", state.0, state.1);
        }
        match_statement += "\t\t_ => None\n";
        match_statement += "\t}\n";
        header += &match_statement;
        header += "}";
        return header;
    }

    pub fn create_transition_function(&mut self) -> String
    {
        let mut header: String = "fn transition(curr: i32, trans: TransitionKind) -> i32\n{\n".to_string();

        // For each kind of key
        for (key, value) in &self.table.transitions
        {
            let mut if_statement = format!("\tif {0} == {1}\n\t{{\n", self.curr_state_name, key);
            let mut char_transition_statement = None;
            let mut any_transition_statement = None;
            
            for (trans_kind, dest) in value
            {
                if let TransitionKind::Character(character) = trans_kind
                {
                    if let None = char_transition_statement {
                        char_transition_statement = Some("\t\tif let TransitionKind::Character(trans_char) = trans\n\t\t{\n".to_string());
                    }

                    let inner_statement = format!("\t\t\tif trans_char == '{0}'\n \
                    \t\t\t{{\n \
                        \t\t\t\treturn {1};\n\
                    \t\t\t}}\n", character, dest);

                    char_transition_statement = Some(char_transition_statement.unwrap() + &inner_statement);
                }
                else 
                {
                    any_transition_statement = Some(format!("if let TransitionKind::AnyCharacter = trans\n \
                    \t\t\t{{ \n \
                        \t\t\t\treturn {0};\n\
                    \t\t\t}}\n", dest));
                }
            }

            // Add '}' to char transition statement
            if char_transition_statement.is_some() {
                char_transition_statement = Some(char_transition_statement.unwrap() + "\t\t}\n");
                if_statement += &char_transition_statement.unwrap();
            }

            if any_transition_statement.is_some() {
                if_statement += &any_transition_statement.unwrap();
            }

            if_statement += "\t}\n";
            header += &if_statement;
        }

        header += "\tpanic!();\n";
        header += "}";
        return header;
    }
}