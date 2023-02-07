# File Format Specification

The file format for RLPG combines the rules for lexing and parsing into a single file. This is done so that the lexer and parser components would work well together as a system in a cohesive manner.

The file format is similar to both those of Flex and Bison albeit intending to be less seemingly glued-together.
The file format is comprised of three sections: code, lexer, and grammar.
The code is section is intended for preliminary code.
The lexer section is intended for rules for the lexical analyzer.
The grammar section is intended for rules for the parser.

Each section starts with the text: "SECTION {capitalized section name}".
Each section ends until the next section is reached.
In the case of the grammar section, it ends at the end of the file.

Only one of each type of section is allowed to appear in a file.
The sections must appear in the proper order (e.g., code comes before defintions, etc.).
Only the lexer section is required to exist.
All other sections, besdies the lexer section, are optional.

Every file must begin with a section header in order to be deemed valid.

## Code Section

The code section contains preliminary Rust code for both the lexer and parser, and is akin to the code section at the beginning of a file in Flex or Bison.

## Lexer Section

The lexer section contains rules for the lexer to operate.
There are two types of rules for the lexer: named and unnamed.

Each named rule begins with a string of characters, followed by whitespace, followed by regular expression to parse.
Named rules must have unique names.
For example:
plus "+"
digit "[0-9]+"
idenifier "[a-zA-Z]"

Each rule may optionally be followed by a code in curly brackets.
This code runs whenever the rule is parsed.
The code should not return anything.
For example:

plus "+" { println!("PLUS");}

Another type of rule is called an unnamed rule, which are ignored and no symbol is produced for it.
Unnamed rules may also have action code.
For example:

"[ \t\n\r]" { println!("Whitespace encountered!"); }

In this example, the whitespace is ignored by the lexer.

The rule . catches anything that did not match any rule.
For example:
plus "+"
digit "[0-9]+"
idenifier "[a-zA-Z]"
.                       { println!("Unknown token"); }


In the case of conflict in determining which rule best fits the lexeme, precedance is given to rules that appear earlier in the file.
This form of conflict resolution occurs for both named and unnamed rules.

For example:
plus "+"
digit "[0-9]+"
idenifier "[a-zA-Z]"
id_number "[0-9]"

Here, if the sequence "1" is encountered, it is interpreted as a digit, not an id_number.

## Grammar Section

This section is intended for rules for the parser.
The grammar is specified in Backus-Naur Form (BNF).
The rules are intended to be specified in the same manner as in Yacc or Bison.

The rules can include string literals.
However, these string literals will be interpreted literally, not as regular expressions.

For example:
number: "[0-9]+"

In this rule, the parser will look for a string that literally matches "[0-9]+" and will not interpret it as a regular expression.

Each rule defintion is written on its own line.
The rule is specified as:
<rule name>: <sequence of one of more rule names or string literals>
;

The semi-colon indicates the end of the rule.

For example:
expression: "(" expression ")
;

If a rule has multiple possible derivations, they are written on the lines following the first derivation in the following manner:

expression: "(" expression ")
| number
| identifier
;

A rule may optionally be followed by code in curly braces similar to Yacc and Bison. For example:

expression: "(" expression ")" { println!("{}", get_argument(1)); }
| number { println!("{}", get_argument(0)); }
| identifier
;

The function "get_argument(x)" is intended to return the runtime value of the x-th part of a derivation.
The API development is still in progress, so the name of the function or its mechanics is not set in stone.

## Supported Regular Expressions

The lexical analyzer is currently intended to support regular expression operators that are critical to the robust support of regular languages.

The following operators are valid and supported:
- "*" (Kleene star)
- "+"
- "."
- "?"
- "(...)" (Parentheses)
- "[...]" (Brackets)
- "[0-9]" (Match any decimal digit)
- "[a-zA-Z]" (Match any English character)
- "|" (OR operator)

There are other regular expression operators that have not been mentioned.
These operators are not inteded to be supported, but may be supported in the future.