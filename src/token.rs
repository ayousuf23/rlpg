use crate::grammar2::Symbol;

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct Token {
    pub lexeme: String,
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub symbol: Symbol,
}

impl Token {
    pub fn new(lexeme: String, start_col: usize, end_col: usize, symbol: Symbol) -> Token
    {
        Token { lexeme: lexeme, line: 0, start_col: start_col, end_col: end_col, symbol: symbol }
    }
}