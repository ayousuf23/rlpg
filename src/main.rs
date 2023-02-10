mod file_parser;
use file_parser::FileParser;

mod regex_parser;
use regex_parser::RegExParser;

use clap::Parser;

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
    let mut parser = RegExParser::new("hello");
    parser.parse();
}
