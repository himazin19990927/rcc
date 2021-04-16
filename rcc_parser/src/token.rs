#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Lt,
    Le,
    EqEq,
    Ne,
    Ge,
    Gt,

    Eq,

    Num,
    Plus,
    Minus,
    Star,
    Slash,
    OpenParen,
    CloseParen,
    Semi,

    Int,

    Identifier,

    EOF,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new<T: Into<String>>(kind: TokenKind, literal: T) -> Token {
        Token {
            kind: kind,
            literal: literal.into(),
        }
    }
}
