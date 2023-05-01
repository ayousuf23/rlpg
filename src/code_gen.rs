use std::{any, io::Write, fmt::format};

use crate::{table_dfa_builder::TableDFA, nfa::TransitionKind, token::Token, grammar2::GrammarGenerator};
use std::fs::File;

pub struct CodeGen
{
    pub table: TableDFA,
    pub curr_state_name: String,
    pub grammar_gen: GrammarGenerator,
}

impl CodeGen {

    fn write_to_file(path: &str, text: String)
    {
        if let Ok(mut file) = File::create(path)
        {
            if let Err(err) = file.write_all(text.as_bytes())
            {

            }
        }   
    }

    pub fn generate_lexer(&mut self, path: &str)
    {
        let mut text = self.create_transition_kind();
        text += "\n";
        text += &self.create_check_accepting_state_function();
        text += "\n";
        text += &self.create_transition_function();
        text += "\n";
        text += &self.create_parse_function();
        text += "\n";
        text += &self.create_main_fn();
        CodeGen::write_to_file(path, text);
    }

    fn create_main_fn(&self) -> String
    {
        return
"pub fn main()
{
    println!(\"Enter a string to match: \");
    let mut to_check = String::new();
    std::io::stdin().read_line(&mut to_check).expect(\"failed to readline\");
    let to_check = to_check.trim().to_string();
    let result = parse(to_check);
    println!(\"{:?}\", result);
}".to_string();
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
        
        // Perform transition or error
        let trans_kind = TransitionKind::Character(seq[index]);
        if let Some(next_state) = transition(curr_state, trans_kind) {
            curr_state = next_state
        } else {
            // if current state is accepting
            if let Some(token) = is_accepting(curr_state)
            {
                if !token.is_empty() {
                    tokens.push(token);
                }
                curr_state = 1;
            }
            else {
                panic!();
            }
        }
        index += 1;
    }
    if let Some(token) = is_accepting(curr_state)
    {
        if !token.is_empty() {
            tokens.push(token);
        }
    }
    tokens
}\n".to_string()
    }

    pub fn create_transition_kind(&mut self) -> String 
    {
        stringify!(
            pub enum TransitionKind {
                Character(char),
                AnyChar,
            }
        ).to_string()
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
        let mut header: String = "fn transition(curr: i32, trans: TransitionKind) -> Option<i32>\n{\n".to_string();

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
                        \t\t\t\treturn Some({1});\n\
                    \t\t\t}}\n", character, dest);

                    char_transition_statement = Some(char_transition_statement.unwrap() + &inner_statement);
                }
                else 
                {
                    any_transition_statement = Some(format!("if let TransitionKind::AnyCharacter = trans\n \
                    \t\t\t{{ \n \
                        \t\t\t\treturn Some({0});\n\
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

        header += "\treturn None;\n";
        header += "}\n";
        return header;
    }

    pub fn get_tree_enum(&self) -> String
    {
        stringify!(
            struct TreeNode {
                pub symbol: Symbol,
                pub children: Vec<Symbol>,
            }
        ).to_string()
    }
}