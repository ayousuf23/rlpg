use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct FileParser;

impl FileParser {
    fn is_valid_section_header(line: &str) -> bool {
        return line == "SECTION LEXER";
    }

    pub fn parse_file(path: &str) {
        let file = File::open(path).expect("Error file cound not be opened!");

        let mut reader = BufReader::new(file);

        let mut first_line: String = String::new();
        reader.read_line(&mut first_line).expect("Error");

        if FileParser::is_valid_section_header(&first_line) {
            println!("Valid");
        }
        else {
            println!("Invalid");
        }
    }
}