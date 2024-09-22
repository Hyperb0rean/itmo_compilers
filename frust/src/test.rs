use crate::io::*;
use crate::preprocessor::*;
use crate::lexer::*;
use crate::parser::*;



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


#[test]
pub fn test_parser() {
    let code = read_file("tests/example.fr".to_string()).unwrap();
    let preprocessed_code = remove_comments(&code);
    let tokens = lexer(&preprocessed_code).unwrap();
    
    let mut parser = Parser::new(tokens);
    let expressions = parser.parse().unwrap();
    for expr in expressions {
        println!("{:#?}", expr);
    }
}