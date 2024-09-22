use std::env;
mod io;
mod ast;
mod lexer;
mod preprocessor;

mod test;

use crate::preprocessor::*;
use io::*;
use lexer::*;

fn main() {
    let (input, _) = parse_args(env::args().collect()).unwrap();
    let code = read_file(input).unwrap();
    let preprocessed_code = remove_comments(&code);

    let _tokens = lexer(&preprocessed_code).unwrap();
}
