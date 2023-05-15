mod file_parser;

pub mod regex_parser;

mod node_kind;

mod nfa_builder;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
use grammar2::{Symbol, Production, GrammarRule, GrammarGenerator};


mod grammar2;

mod table_dfa_builder;

mod code_gen;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to parse
    #[arg(long)]
    filename: String,
    /// Path of the output file
    #[arg(long)]
    output: String,
}

fn main() {

    let args = Args::parse();

    let mut file_parser = FileParser::new();

    // Open the file and parse it
    let file_parse_result = file_parser.parse_file(&args.filename);
    if let Err(error) = file_parse_result
    {
        println!("{}", format!("Error: {}", error.to_string()).red());
        return;
    }
    //let rules = file_parse_result.unwrap();


    // Take the rules and build an NFA
    unsafe {
        let nfa = file_parser.build_nfa();
        if let Err(error) = nfa
        {
            println!("{}", format!("Error: {}", error.to_string()).red());
            return;
        }
        let nfa = nfa.unwrap();
        //println!("{:?}", map);
        let dfa = dfa_builder::DFABuilder::convert_nfa_to_dfa(nfa);

        // Generate table dfa
        let mut table_builder = TableDFABuilder {
            mapping: HashMap::new(),
            node_counter: 1,
        };

        let table = table_builder.build_table_dfa(dfa);

        // Set emptiness info for terminal symbols
        let terminals = file_parser.get_terminals();
        println!("terminals: {:#?}", terminals);

        // Create grammar generator
        let mut grammar_gen = grammar2::GrammarGenerator::new(file_parser.get_terminals());
     
        for rule in file_parser.grammar_rules {
            println!("Rule: {}", rule.name);
            for item in &rule.productions {
                for sym in &(**item).prod {
                    print!("{}({:?}) ", sym.name, sym.emptiness);
                }
            }
            println!();
            let symbol = grammar2::Symbol { name: rule.name.to_string(), is_terminal: false, emptiness: crate::grammar2::Empty::NonEmpty };
            
            grammar_gen.add_rule(symbol, rule);
        }
        let cc = grammar_gen.build_cannocial_collection();

        // Print cc
        /*for (set, info) in cc {
            println!("Item {}", info.id);
            for item in &set.set {
                println!("{}", **item);
            }   
        }*/

        // Fill table
        grammar_gen.build_table(&cc);

        /*let one = Symbol {
            name: "number".to_string(),
            is_terminal: true,
            emptiness: grammar2::Empty::NonEmpty,
        };
        let space = Symbol {
            name: "space".to_string(),
            is_terminal: true,
            emptiness: grammar2::Empty::PossiblyEmpty,
        };
        let plus = Symbol {
            name: "plus".to_string(),
            is_terminal: true,
            emptiness: grammar2::Empty::NonEmpty,
        };
        let symbols = vec![one.clone(), plus, one, crate::grammar2::Symbol::eof_symbol()];
        println!("{}", grammar_gen.parse(&symbols));*/

        // Code gen
        let mut code_gen = CodeGen {
            table: table,
            curr_state_name: "curr".to_string(),
            grammar_gen: grammar_gen,
        };

        //let path = std::path::Path::

        match PathBuf::from_str(&args.output) {
            Ok(path) => {
                if path.exists() {
                    println!("{}", format!("Error: The path {} already exists. Please delete it then try again.", path.to_str().unwrap()).red());
                }
                else {
                    if let Err(error) = code_gen.generate_lexer(path.to_str().unwrap())
                    {
                        println!("{}", format!("Error: {}", error.to_string()).red());
                    }
                }
                
            },
            // This case is considered infalliable
            Err(_) => (),
        }
    }
}
