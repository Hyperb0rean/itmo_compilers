
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
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
    #[token("i64")]
    TypeI64,
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
    #[token("true")]
    LitTrue,
    #[token("false")]
    LitFalse,

    #[regex(r"\d+", |lex| lex.slice().parse().map_err(|_| ()))]
    LitNumber(i64),
}

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => tokens.push(token),
            _ => return Err(format!("Unknown lexem")),
        }
    }

    Ok(tokens)
}
