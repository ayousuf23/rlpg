# RLPG
RLPG stands for Rust Lexer and Parser Generator.
As the name states, RLPG is a lexical analyzer and parser generator written in Rust.

## Documentation

The docs folder contains two documents specifying the overall specification for RLPG.
This includes console commands, file format, and regular expression support description among others.
Due to the nature of the project and its current stage, the documentation is not intended to be highly detailed regarding the source code.
Rather, the documentation is meant to provide a high-level overview of the necessary mechanisms used in the project and their specific details that would be relevant to usage.

## Build Instructions

Prerequisites:
- Install Rust

In the project folder (i.e., the base directory of the repo), run `cargo build` to build the project.

The RLPG program takes in two parameters, one for the source file and one for the output file (output produced by the program).
To run the program, enter `cargo run -- --filename <source path> --output <output path>` on the CLI.

## Test Instructions

The project uses tests that depend on reading example files (that mimic possible types of erroneous or non-erroneous input the program may face) from the filesystem.

To run the tests, enter the command `cargo test`.
