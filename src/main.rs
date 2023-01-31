mod node;
use node::Node;
use std::io;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to parse
    #[arg(short, long)]
    filename: String,
}

fn main() {

    let args = Args::parse();

    println!("{}", args.filename)    

    /*println!("Hello!");

    // Create the node
    let mut parent = Node::new("S".to_string());

    // Read input from command line
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let mut prev = &mut parent;

    // Loop through each character
    for c in line.trim().chars() {
        println!("Character {c}");

        // Create a node for the character
        let mut node = Box::new(Node::new(c.to_string()));

        prev.child = Some(node);

        //let mut inside = prev.child.unwrap();
        prev = prev.child.as_mut().unwrap();

    }

    // Print out the contents of the linked list
    prev = &mut parent;
    while let Some(i) = &mut prev.child {
        let node = i.as_mut();
        print!("-{0}", node.data);
        prev = node;//.child.as_mut().unwrap().as_mut();
    }
    print!("\n");*/

}
