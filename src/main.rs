mod file_parser;

pub mod regex_parser;

mod node_kind;

mod nfa_builder;
use std::collections::HashMap;

use crate::error::RlpgErr;
use crate::file_parser::FileParser;
pub use crate::nfa_builder::NFABuilder;

pub mod nfa;
pub use crate::nfa::NFA;

pub mod token;

use clap::Parser;

pub mod node;

pub mod error;

mod tests;

mod dfa_builder;

use colored::*;
use dfa_builder::DFANode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to parse
    #[arg(short, long)]
    filename: String,
}

fn main() {

    let args = Args::parse();

    let mut file_parser = FileParser {
        rules: Vec::new(),
    };

    // Open the file
    if let Err(error) = file_parser.parse_file(&args.filename)
    {
        println!("{}", error.get_err_message().red());
        return;
    }

    // Take the rules and build an NFA
    let nfa = NFA::build_from_rules(&file_parser.rules).unwrap();

    // Create a DFA 
    let dfa_builder = dfa_builder::DFABuilder {
        nodes: HashMap::new(),
    };
    let dfa = dfa_builder.convert_nfa_to_dfa(nfa);

    // Print 1st node
    DFANode::print(dfa);

    // Simulate on text!
    /*loop {
        println!("Enter a string to match: ");
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();

        let (result, tokens) = nfa.simulateAndGetToken(&to_check);
        for token in tokens {
            println!("Token: {} (Lexeme: {}, Line: {}, Columns: {}-{})", token.name, token.lexeme, token.line, token.start_col, token.end_col);
        }

        if result {
            println!("Result: Success");
        }
        else {
            println!("Result: Failure");
        }
    }*/

    //println!("{:?}", nfa);
}
