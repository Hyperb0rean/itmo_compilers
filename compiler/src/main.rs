use std::env;

use io::{parse_args, read_file};

mod io;

fn main() {
    let (input, output) = parse_args(env::args().collect()).unwrap();
    let code = read_file(input).unwrap();
}
