use crate::io::*;
use crate::preprocessor::*;

#[test]
pub fn test_preprocessing() {
    let code = read_file("tests/example.fr".to_string()).unwrap();
    let preprocessed_code = remove_comments(&code);
    println!("{}", preprocessed_code);
}
