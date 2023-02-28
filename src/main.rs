mod file_parser;

pub mod regex_parser;

mod node_kind;

mod nfa_builder;
use crate::file_parser::FileParser;
pub use crate::nfa_builder::NFABuilder;

pub mod nfa;
pub use crate::nfa::NFA;

use clap::Parser;

pub mod node;

mod tests;

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
        panic!("{:?}", error.kind);
    }

    // Take the rules and build an NFA
    let nfa = NFA::build_from_rules(&file_parser.rules).unwrap();

    // Simulate on text!

    /*let mut regex = String::new();
    println!("Enter regular expression: ");
    std::io::stdin().read_line(&mut regex).expect("failed to readline");
    let regex = regex.trim().to_string();

    // Create a regex parser
    let mut parser = RegExParser::new(&regex);
    let mut parse_root = parser.parse();

    // Generate an NFA
    let mut nfa = NFABuilder::build(&parse_root).expect("Error");
    */

    loop {
        println!("Enter a string to match: ");
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();

        if let Some(token) = nfa.simulateAndGetToken(&to_check) {
            println!("Result: Success ({})", token);
        }
        else {
            println!("Result: Failure");
        }
    }

    //println!("{:?}", nfa);
}
