use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    ERROR,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    NUMBER(i64),
    IDENT(String),
    KEYWORD(String),
    UNARY(char),
}

pub struct Token {
    pub literal: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(literal: String, kind: TokenKind) -> Self {
        Token { literal, kind }
    }
}
