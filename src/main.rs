mod file_parser;
use std::io;

use file_parser::FileParser;

mod regex_parser;
use regex_parser::RegExParser;

mod node_kind;
use node_kind::NodeKind;

mod nfa_builder;

mod nfa;

use clap::Parser;

use crate::nfa_builder::NFABuilder;

mod node;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to parse
    #[arg(short, long)]
    filename: String,
}

fn main() {

    let args = Args::parse();

    println!("{}", args.filename);    

    // Open the file
    FileParser::parse_file(&args.filename);

    // Create a regex parser
    let mut parser = RegExParser::new("hell.");
    let mut parse_root = parser.parse();

    // Generate an NFA
    let mut nfa = NFABuilder::build(&parse_root).expect("Error");

    loop {
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();

        if nfa.simulate(to_check) {
            println!("Success");
        }
        else {
            println!("Failure");
        }
    }

    //println!("{:?}", nfa);
}
