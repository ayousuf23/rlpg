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
use crate::grammar::LRItem;
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
use grammar::{Symbol, Production, GrammarRule, GrammarGenerator};

mod grammar;
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

        /*println!("Enter a string to match: ");
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();

        let result = DFASimulator::simulate_dfa_and_get_tokens(dfa, &to_check);
        println!("{:?}", result.0);
        println!("{:?}", result.1);*/

        // Generate table dfa
        let mut table_builder = TableDFABuilder {
            mapping: HashMap::new(),
            node_counter: 1,
        };

        let table = table_builder.build_table_dfa(dfa);

        // Code gen
        /*let mut code_gen = CodeGen {
            table: table,
            curr_state_name: "curr".to_string(),
        };

        // Dir
        if let Ok(path) = std::env::current_dir() {
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

        // Create grammar generator
        let mut grammar_gen = grammar2::GrammarGenerator::new();
        for rule in file_parser.grammar_rules {
            let symbol = grammar2::Symbol { name: rule.name.to_string(), is_terminal: false };
            grammar_gen.add_rule(symbol, rule);
        }

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

        let cc = grammar_gen.build_cannocial_collection();
        println!("{}", cc.len());

        //println!("Start");
        for set in (cc).keys() {
            println!("Start");
            for prod in &set.set {
                println!("{}", **prod);
            }
            println!("End");
        }
        //println!("{:?}", **item);
        //println!("End");

        // Fill table
        grammar_gen.build_table(&cc);

        println!("{:?}", grammar_gen.action_table);

        println!("{}", grammar_gen.parse());

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


        // Create a plus symbol
        /*let plus_symbol = Symbol {name: "+".to_string(), is_terminal: true};
        let nt1_symbol = Symbol {name: "NT1".to_string(), is_terminal: false};
        let nt2_symbol = Symbol {name: "NT2".to_string(), is_terminal: false};

        // create prods
        let mut nt1_prod = Production {
            prod: Vec::new(),
        };
        nt1_prod.prod.push(plus_symbol.clone());
        let nt1_prod_raw = Box::into_raw(Box::new(nt1_prod));
        
        let mut nt2_prod = Production {
            prod: Vec::new(),
        };
        nt2_prod.prod.push(nt1_symbol.clone());
        let nt2_prod_raw = Box::into_raw(Box::new(nt2_prod));

        // create rules
        let mut nt1_rule = GrammarRule {
            name: "NT1".to_string(),
            productions: Vec::new(),
        };
        nt1_rule.productions.push(nt1_prod_raw);

        let mut nt2_rule = GrammarRule {
            name: "NT2".to_string(),
            productions: Vec::new(),
        };
        nt2_rule.productions.push(nt2_prod_raw);

        // create rule set
        let mut grammar_gen = GrammarGenerator::new();
        grammar_gen.add_rule(nt1_symbol.clone(), nt1_rule);
        grammar_gen.add_rule(nt2_symbol.clone(), nt2_rule);
       
        let mut set = HashSet::new();
        let str = vec![nt1_symbol.clone()];
        grammar_gen.get_first_set(&str, &mut set);
        //println!("{:?}", set);

        // Create LR Item
        let lr_item = LRItem { production: nt2_prod_raw, placeholder_index: 0, lookup_sym: Symbol { name: "eof".to_string(), is_terminal: true }};
        let mut clos_set = HashSet::new();
        clos_set.insert(lr_item);

        let result = grammar_gen.get_closure(&clos_set);
        for x in &result {
            println!("Production: {:?}, lookup: {:?}", *(x.production), x.lookup_sym);
        }
        println!("{:?}", result);*/
    }
}
