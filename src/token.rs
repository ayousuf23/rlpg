pub struct Token {
    pub name: String,
    pub lexeme: String,
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
}