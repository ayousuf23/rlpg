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

    // Open the file
    if let Err(error) = FileParser::parse_file(&args.filename)
    {
        panic!();
    }

    /*let mut regex = String::new();
    println!("Enter regular expression: ");
    std::io::stdin().read_line(&mut regex).expect("failed to readline");
    let regex = regex.trim().to_string();

    // Create a regex parser
    let mut parser = RegExParser::new(&regex);
    let mut parse_root = parser.parse();

    // Generate an NFA
    let mut nfa = NFABuilder::build(&parse_root).expect("Error");

    loop {
        println!("Enter a string to match: ");
        let mut to_check = String::new();
        std::io::stdin().read_line(&mut to_check).expect("failed to readline");
        let to_check = to_check.trim().to_string();

        if nfa.simulate(&to_check) {
            println!("Result: Success");
        }
        else {
            println!("Result: Failure");
        }
    }*/

    //println!("{:?}", nfa);
}
