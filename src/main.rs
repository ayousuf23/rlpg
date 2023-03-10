mod file_parser;

pub mod regex_parser;

mod node_kind;

mod nfa_builder;
use std::collections::HashMap;

use crate::dfa_builder::DFABuilder;
use crate::dfa_simulator::DFASimulator;
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

mod dfa_simulator;

use colored::*;
use dfa_builder::DFANode;

mod table_dfa_builder;

mod code_gen;

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
    unsafe {
        let nfa = NFA::build_from_rules(&file_parser.rules).unwrap();

        // Create a DFA 
        let mut dfa_builder = dfa_builder::DFABuilder {
            nodes: HashMap::new(),
            raw_nodes: HashMap::new(),
        };
        let dfa = dfa_builder.convert_nfa_to_dfa_raw(nfa);

        // Print 1st node
        //DFANode::print(dfa);

        println!("Enter a string to match: ");
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();

        DFASimulator::simulate_dfa(dfa, to_check.chars().collect());
    }

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
