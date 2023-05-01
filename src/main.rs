mod file_parser;

pub mod regex_parser;

mod node_kind;

mod nfa_builder;
use std::collections::{HashMap, HashSet};
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
use grammar2::{Symbol, Production, GrammarRule, GrammarGenerator};


mod grammar2;

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

    let mut file_parser = FileParser::new();

    // Open the file
    let file_parse_result = file_parser.parse_file(&args.filename);
    if let Err(error) = file_parse_result
    {
        println!("{}", error);
        return;
    }
    let rules = file_parse_result.unwrap();


    // Take the rules and build an NFA
    unsafe {
        let nfa = NFA::build_from_rules(&rules);
        if nfa.is_err()
        {
            println!("{}", nfa.err().unwrap());
            return;
        }

        let dfa = dfa_builder::DFABuilder::convert_nfa_to_dfa(nfa.unwrap());

        // Print 1st node
        //DFANode::print(dfa);

        println!("Enter a string to match: ");
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();

        let (result, mut symbols) = DFASimulator::simulate_dfa_and_get_tokens(dfa, &to_check);
        if !result {
            panic!();
        }
        symbols.push(Symbol::eof_symbol());
        println!("{:?}", symbols);

        // Generate table dfa
        let mut table_builder = TableDFABuilder {
            mapping: HashMap::new(),
            node_counter: 1,
        };

        let table = table_builder.build_table_dfa(dfa);

        // Create grammar generator
        let mut grammar_gen = grammar2::GrammarGenerator::new(file_parser.get_terminals());
     
        for rule in file_parser.grammar_rules {
            let symbol = grammar2::Symbol { name: rule.name.to_string(), is_terminal: false };
            grammar_gen.add_rule(symbol, rule);
        }
        let cc = grammar_gen.build_cannocial_collection();

        // Fill table
        grammar_gen.build_table(&cc);

        // Code gen
        let code_gen = CodeGen {
            table: table,
            curr_state_name: "curr".to_string(),
            grammar_gen: grammar_gen,
        };

        // Dir
        /*if let Ok(path) = std::env::current_dir() {
            let cur_dir = path.join(Path::new("result.rs"));

            code_gen.generate_lexer(cur_dir.to_str().unwrap());
        }*/


        // Print grammar rules
        //println!("{:?}", file_parser.grammar_rules);
        /*for rule in &file_parser.grammar_rules {
            println!("Name: {}", rule.name);
            for prod in &rule.productions {
                println!("{:?}", (**prod).prod);
            }
        }*/

        

        // goal
        //let goal = grammar_gen.get_goal_grammar_set();
        //let c = grammar_gen.get_closure(goal);

        // do goto
        //let goto = grammar_gen.get_goto(&c, &grammar2::Symbol{is_terminal: true, name: "left".to_string()});

        /*println!("Start");
        for prod in &(goto).set {
            println!("{}", **prod);
        }
        //println!("{:?}", **item);
        println!("End");*/

        
        //println!("{}", cc.len());

        //println!("Start");
        /*for set in (cc).keys() {
            println!("Start");
            for prod in &set.set {
                println!("{}", **prod);
            }
            println!("End");
        }*/
        //println!("{:?}", **item);
        //println!("End");

        


        println!("{}", code_gen.get_tree_enum());

        /*let result = grammar_gen.build_cannocial_collection();
        let mut i = 0;
        for item in &result {
            println!("Start");
            for prod in &(**item).set {
                println!("{}", prod);
            }
            //println!("{:?}", **item);
            println!("End");
        }*/
        //println!("{:?}", result);
    }
}
