use std::{default, num::ParseIntError};

use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidNumber(String),
    #[default]
    UnknownLexem,
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidNumber("Overflow error".to_owned()),
            _ => LexingError::InvalidNumber("Other error".to_owned()),
        }
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // Key words
    #[token("let")]
    KwLet,
    #[token("if")]
    KwIf,
    #[token("else")]
    KwElse,
    #[token("while")]
    KwWhile,

    // Types
    #[token("num")]
    TypeNumber,
    #[token("bool")]
    TypeBool,

    // Operators
    #[token("==")]
    OpEq,
    #[token("!=")]
    OpNeq,
    #[token("<=")]
    OpLe,
    #[token(">=")]
    OpGe,
    #[token("&&")]
    OpAnd,
    #[token("||")]
    OpOr,
    #[token("+")]
    OpAdd,
    #[token("-")]
    OpSub,
    #[token("*")]
    OpMul,
    #[token("/")]
    OpDiv,
    #[token("<")]
    OpLt,
    #[token(">")]
    OpGt,
    #[token("!")]
    OpNot,

    // Other
    #[token("=")]
    Assign,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Literals
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    LitBool(bool),

    #[regex(r"\d+", |lex| lex.slice().parse())]
    LitNumber(i64),
}

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => tokens.push(token),
            Err(LexingError::InvalidNumber(s)) => return Err(format!("Invalid number: {}", s)),
            _ => return Err(format!("Unknown lexem")),
        }
    }

    Ok(tokens)
}
