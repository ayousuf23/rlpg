use std::any;

use crate::{table_dfa_builder::TableDFA, nfa::TransitionKind};

pub struct CodeGen
{
    pub table: TableDFA,
    pub curr_state_name: String,
}

impl CodeGen {

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

        header += "}";
        return header;
    }
}