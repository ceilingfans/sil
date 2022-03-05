#[derive(Debug)]
pub struct Loc {
    pub position: u64,
    pub line: u64,
}

#[derive(Debug)]
pub enum TokenType {
    INVALID,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    BANG,
    PIPE,
    NUMBER(i64),
    IDENT(String),
    KEYWORD(String),
    UNARY(char),
}

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub literal: String,
    pub location: Loc,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            ty: TokenType::INVALID,
            literal: String::new(),
            location: Loc { position: 0, line: 0 }
        }
    }
}

impl Token {
    pub fn new() -> Self {
        Default::default()
    }
}
