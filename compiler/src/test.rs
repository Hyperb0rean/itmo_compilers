use crate::io::*;
use crate::preprocessor::*;
use crate::lexer::*;



#[test]
pub fn test_preprocessing() {
    let code = read_file("tests/example.fr".to_string()).unwrap();
    let preprocessed_code = remove_comments(&code);
    println!("{}", preprocessed_code);
}

#[test]
pub fn test_lexer() {
    let code = read_file("tests/example.fr".to_string()).unwrap();
    let preprocessed_code = remove_comments(&code);
    let tokens = lexer(&preprocessed_code).unwrap();

    for token in &tokens {
        println!("{:?}", token);
    }
}
