use crate::grammar2::Symbol;

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct Token {
    pub name: String,
    pub lexeme: String,
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub symbol: Symbol,
}
