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

    fn write_to_file(path: &str, text: String) -> std::io::Result<()>
    {
        match File::create(path) {
            Ok(mut file) => {
                if let Err(error) = file.write_all(text.as_bytes())
                {
                    return Err(error);
                }
            },
            Err(error) => return Err(error),
        }
        return Ok(());
    }

    pub fn generate_lexer(&mut self, path: &str) -> std::io::Result<()>
    {
        let mut text = "use std::collections::HashMap;\n".to_string();
        text += "\n\n";

        text += &self.create_structs_and_enums();
        text += "\n";

        text += &self.create_transition_kind();
        text += "\n";
        text += &self.create_check_accepting_state_function();
        text += "\n";
        text += &self.create_transition_function();
        text += "\n";
        text += &self.create_get_action_table_function();
        text += "\n";
        text += &self.create_get_goto_table_function();
        text += "\n";
        text += &self.create_parse_function();
        text += "\n";
        text += &self.create_grammar_parse_function();
        text += "\n";
        text += &self.create_main_fn();
        
        return CodeGen::write_to_file(path, text);
    }

    fn create_main_fn(&self) -> String
    {
        let mut main = "pub fn main()
        {{
            println!(\"Enter a string to match: \");
            let mut to_check = String::new();
            std::io::stdin().read_line(&mut to_check).expect(\"failed to readline\");
            let to_check = to_check.trim().to_string();
            let result = get_tokens(to_check);
            //println!(\"{:?}\", result);
            if let Err(err) = result {
                println!(\"Error\");
                return;
            }

            let mut result = result.unwrap();
            let eof_token = Token::new(\"eof\".to_string(), 0, 0, Symbol::eof_symbol());
            result.push(eof_token);

        ".to_string();

        //main += &format!("let action_table = {}", self.create_action_table());

        //main += &format!("let goto_table = {}", self.create_goto_table());

        main += "let grammar_result = parse(&result);";
        main += "println!(\"Result: {}\", grammar_result.is_err())";

        main += "}}";
        return main;
    }

    pub fn create_parse_function(&mut self) -> String
    {
        stringify!(
            pub fn get_tokens(text: String) -> Result<Vec<Token>, ErrorKind>
            {
                let mut curr_state = 1;
                let seq: Vec<char> = text.chars().collect();
                let mut index = 0;
                let mut tokens: Vec<Token> = Vec::new();

                // Used to keep track of lexeme info
                let mut start_col = 0;
                let mut end_col = 0;

                while index < seq.len() {
                    // Check if accepting state
                    
                    // Perform transition or error
                    let trans_kind = TransitionKind::Character(seq[index]);
                    if let Some(next_state) = transition(curr_state, trans_kind) {
                        curr_state = next_state;
                        end_col += 1;
                    } else {
                        // if current state is accepting
                        if let Some(token) = is_accepting(curr_state)
                        {
                            //println!("token index {} {}", index, curr_state);
                            if !token.is_empty() {
                                let sym = Symbol {name: token.to_string(), is_terminal: true};
                                let lexeme = text[start_col..end_col].to_string();
                                let token = Token::new(lexeme, start_col, end_col - 1, sym);
                                tokens.push(token);
                            }
                            curr_state = 1;
                            start_col = index;
                            end_col = index;
                            index -= 1;
                        }
                        else {
                            return Err(ErrorKind::TokenizationFailed(start_col, end_col));
                        }
                    }
                    index += 1;
                }

                //println!("token index {} {}", index, curr_state);
                if let Some(token) = is_accepting(curr_state)
                {
                    if !token.is_empty() {
                        let sym = Symbol {name: token.to_string(), is_terminal: true};
                        let lexeme = text[start_col..end_col].to_string();
                        let token = Token::new(lexeme, start_col, end_col - 1, sym);
                        tokens.push(token);
                    }
                }
                let eof_token = Token::new("eof".to_string(), end_col, end_col, Symbol::eof_symbol());
                tokens.push(eof_token);
                Ok(tokens)
            }

        ).to_string()
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

    fn create_grammar_parse_function(&self) -> String
    {
        stringify!(
            pub fn parse(symbols: &Vec<Token>) -> Result<TreeNode, ErrorKind> {
                
                let action_table = get_action_table();
                let goto_table: HashMap<(usize, Symbol), usize> = get_goto_table();

                let mut stack: Vec<StackSymbol> = Vec::new();
                stack.push(StackSymbol::DollarSign);
                stack.push(StackSymbol::State(0));
        
                let mut word = symbols[0].symbol.clone();
                let mut word_index = 0;

                let mut node_children = vec![TreeNode {token: symbols[0].clone(), children: Vec::new()}];

                let mut start_col = 0;
                let mut end_col = 0;
        
                loop {
                    let state = match &stack[stack.len() - 1] {
                        StackSymbol::State(value) => *value,
                        StackSymbol::Symbol(_) => panic!(),
                        StackSymbol::DollarSign => panic!(),
                    };
                    
                    let key = (state, word.clone());
                    
                    if let Some(action) = action_table.get(&key).clone() {
                        
                        match action {
                            Action::Reduce(lhs, prod_len) => {
                                let num = 2 * prod_len;
                                for i in 0..num {
                                    stack.pop();
                                }
                                let state = match &stack[stack.len() - 1] {
                                    StackSymbol::State(value) => *value,
                                    StackSymbol::Symbol(_) => panic!(),
                                    StackSymbol::DollarSign => panic!(),
                                };
                                stack.push(StackSymbol::Symbol(lhs.clone()));
                                let goto = match goto_table.get(&(state, lhs.clone())) {
                                    Some(value) => value,
                                    None => panic!(),
                                };
                                stack.push(StackSymbol::State(*goto));

                                let token = Token::new(lhs.name.to_string(), 0, 0, lhs.clone());
                                let node = TreeNode {token: token, children: node_children};
                                node_children = vec![node];
                                
                            },
                            Action::Shift(dest) => {
                                stack.push(StackSymbol::Symbol(word));
                                stack.push(StackSymbol::State(*dest));
                                word_index += 1;
                                word = symbols[word_index].symbol.clone();
                                let token = Token::new(symbols[word_index].lexeme.to_string(), symbols[word_index].start_col, symbols[word_index].end_col, word.clone());
                                let node = TreeNode {token: token, children: Vec::new()};
                                node_children.push(node);
                            },
                            Action::Accept => break,
                        }
        
                    }
                    else {
                        return Err(ErrorKind::GrammarParseFailed);
                    }
                }

                let root_node = TreeNode {token: Token::new("root".to_string(), 0, 0, Symbol {name: "root".to_string(), is_terminal: false}), children: node_children};
                return Ok(root_node);
            }
        ).to_string()
    }

    fn create_get_action_table_function(&self) -> String
    {
        let mut func = "fn get_action_table() -> HashMap<(usize, Symbol), Action> {\n".to_string();
        func += "return ";
        func += &self.create_action_table();
        func += "}\n";
        return func;
    }

    fn create_get_goto_table_function(&self) -> String
    {
        let mut func = "fn get_goto_table() -> HashMap<(usize, Symbol), usize> {\n".to_string();
        func += "return ";
        func += &self.create_goto_table();
        func += "}\n";
        return func;
    }

    fn create_action_table(&self) -> String
    {
        let mut table = "HashMap::from([\n".to_string();
        for (key, value) in &self.grammar_gen.action_table {
            let action_str = match value {
                crate::grammar2::Action::Accept => "Action::Accept".to_string(),
                crate::grammar2::Action::Shift(state) => format!("Action::Shift({})", state),
                crate::grammar2::Action::Reduce(symbol, size) => format!("Action::Reduce(Symbol {{name: \"{}\".to_string(), is_terminal: {}}}, {})", symbol.name, symbol.is_terminal, size),
            };
            table += &format!("\t(({}, Symbol {{name: \"{}\".to_string(), is_terminal: {} }}), {}),\n", key.0, key.1.name, key.1.is_terminal, action_str);
        }
        table += "]);\n";
        return table;
    }

    fn create_goto_table(&self) -> String
    {
        let mut table = "HashMap::from([\n".to_string();
        for (key, value) in &self.grammar_gen.goto_table {
            table += &format!("(({}, Symbol {{name: \"{}\".to_string(), is_terminal: {} }}), {}),\n", key.0, key.1.name, key.1.is_terminal, value);
        }
        table += "]);\n";
        return table;
    }

    // Structs & enums
    fn create_structs_and_enums(&self) -> String
    {
        let mut text = self.create_symbol_struct();
        text += "\n";
        text += &self.create_token_struct();
        text += "\n";
        text += &self.create_action_struct();
        text += "\n";
        text += &self.create_stack_symbol_struct();
        text += "\n";
        text += &self.create_tree_node_struct();
        text += "\n";
        text += &self.create_error_enum();
        text += "\n";
        return text;
    }

    fn create_token_struct(&self) -> String {
        stringify!(
            #[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
            pub struct Token {
                pub lexeme: String,
                pub line: usize,
                pub start_col: usize,
                pub end_col: usize,
                pub symbol: Symbol,
            }

            impl Token {
                pub fn new(lexeme: String, start_col: usize, end_col: usize, symbol: Symbol) -> Token
                {
                    Token { lexeme: lexeme, line: 0, start_col: start_col, end_col: end_col, symbol: symbol }
                }
            }
        ).to_string()
    }

    fn create_symbol_struct(&self) -> String {
        stringify!(
            #[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
            pub struct Symbol {
                pub name: String,
                pub is_terminal: bool,
            }

            impl Symbol {
            pub fn eof_symbol() -> Symbol
            {
                    Symbol { name: "eof".to_string(), is_terminal: true }
            }
            }
        ).to_string()
    }

    fn create_action_struct(&self) -> String {
        stringify!(
            #[derive(Debug)]
            pub enum Action {
                Shift(usize),
                // LHS and length of production
                Reduce(Symbol, usize),
                Accept
            }
        ).to_string()
    }

    fn create_stack_symbol_struct(&self) -> String
    {
        stringify!(
            enum StackSymbol {
                Symbol(Symbol),
                State(usize),
                DollarSign,
            }
        ).to_string()
    }

    fn create_tree_node_struct(&self) -> String
    {
        stringify!(
            #[derive(Debug)]
            pub struct TreeNode {
                pub token: Token,
                pub children: Vec<TreeNode>,
            }
        ).to_string()
    }

    fn create_error_enum(&self) -> String
    {
        stringify!(
            #[derive(Debug)]
            pub enum ErrorKind {
                GrammarParseFailed,
                TokenizationFailed(usize, usize),
            }

            impl ErrorKind {
                pub fn get_err_message(&self) -> String
                {
                    return match self  {
                        Self::GrammarParseFailed => "Error: the token sequence is not accepted by the grammar".to_string(),
                        Self::TokenizationFailed(start, end) => format!("Error: unable to tokenize the sequence of characters starting at {} and ending at {}", start, end),
                    }
                }
            }
        ).to_string()
    }

}