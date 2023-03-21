mod file_parser;

pub mod regex_parser;

mod node_kind;

mod nfa_builder;
use std::collections::HashMap;
use std::path::Path;

use crate::code_gen::CodeGen;
use crate::dfa_builder::DFABuilder;
use crate::dfa_simulator::DFASimulator;
use crate::error::RlpgErr;
use crate::file_parser::FileParser;
pub use crate::nfa_builder::NFABuilder;

pub mod nfa;
pub use crate::nfa::NFA;
use crate::table_dfa_builder::TableDFABuilder;

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
        let nfa = NFA::build_from_rules(&file_parser.rules);
        if nfa.is_err()
        {
            println!("{}", format!("Error: {}", nfa.err().unwrap()).red());
            return;
        }

        let dfa = dfa_builder::DFABuilder::convert_nfa_to_dfa(nfa.unwrap());

        // Print 1st node
        //DFANode::print(dfa);

        /*println!("Enter a string to match: ");
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();*/

        //DFASimulator::simulate_dfa(dfa, to_check.chars().collect());

        // Generate table dfa
        let mut table_builder = TableDFABuilder {
            mapping: HashMap::new(),
            node_counter: 1,
        };

        let table = table_builder.build_table_dfa(dfa);

        // Code gen
        let mut code_gen = CodeGen {
            table: table,
            curr_state_name: "curr".to_string(),
        };

        // Dir
        if let Ok(path) = std::env::current_dir() {
            let cur_dir = path.join(Path::new("result.rs"));

            code_gen.generate_lexer(cur_dir.to_str().unwrap());
        }

       
    }
}
