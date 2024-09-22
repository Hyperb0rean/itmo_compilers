use std::env;
mod io;
mod preprocessor;

mod test;

use crate::preprocessor::*;
use io::*;

fn main() {
    let (input, output) = parse_args(env::args().collect()).unwrap();
    let code = read_file(input).unwrap();
    let preprocessed_code = remove_comments(&code);
}
