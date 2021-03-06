use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    // Brackets
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,

    // Unary
    #[token("=")]
    Assign,
    #[token("+")]
    Add,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("!")]
    Bang,

    // Other
    #[token("|")]
    Pipe,
    #[token("&")]
    And,
    #[token(":")]
    Colon,
    #[token(";")]
    End,

    // Literals
    #[regex("[0-9]+")]
    Number,
    #[regex("[_a-zA-Z][_0-9a-zA-Z]*")]
    Ident,
    #[regex("(fn|let|while|if)")]
    Keyword,
    #[regex(r#""(?:[^"]|\\")*""#)]
    String,
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
