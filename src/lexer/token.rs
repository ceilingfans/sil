use std::fmt::{Display, Formatter};

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
    IDENT,
    KEYWORD,
    BANG,
    PIPE,
    NUMBER,
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

    pub fn set_location(mut self, loc: Loc) -> Self {
        self.location = loc;
        self
    }

    pub fn set_type(mut self, ty: TokenType) -> Self {
        self.ty = ty;
        self
    }

    pub fn set_literal(mut self, literal: String) -> Self {
        self.literal = literal;
        self
    }
}

