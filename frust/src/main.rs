use std::env;
mod io;
mod ast;
mod lexer;
mod preprocessor;
mod parser;

mod test;

use crate::preprocessor::*;
use ast::Expr;
use io::*;
use lexer::*;
use parser::*;

fn main() {
    let (input, _) = parse_args(env::args().collect()).unwrap();
    let code = read_file(input).unwrap();
    let preprocessed_code = remove_comments(&code);

    let tokens = lexer(&preprocessed_code).unwrap();
    let mut parser = Parser::new(tokens);
    let expressions = parser.parse().unwrap();
    for expr in expressions {
        println!("{:?}", expr);
    }
}
