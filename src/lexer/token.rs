use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[regex("[0-9]+")]
    Number,
    #[regex("[_a-zA-Z][_0-9a-zA-Z]*")]
    Ident,
    #[regex("(fn|let|while|if)")]
    Keyword,
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(literal: String, kind: TokenKind) -> Self {
        Token { literal, kind }
    }
}
