use lazy_static::lazy_static;
use regex::Regex;

pub fn remove_comments(input: &str) -> String {
    lazy_static! {
        // Regex for comments
        static ref RE: Regex = Regex::new(r"(/\*[\s\S]*?\*/)|(//.*)").unwrap();
    }
    RE.replace_all(input, "").to_string()
}
