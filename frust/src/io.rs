use std::fs;

pub fn parse_args(args: Vec<String>) -> Result<(String, String), String> {
    match args.len() {
        3 => Ok((args[1].clone(), args[2].clone())),
        2 => Ok((args[1].clone(), "a.S".to_string())),
        _ => Err("Wrong number of arguments: \n Usage: frustc input.fr output.S".to_string()),
    }
}

pub fn read_file(filename: String) -> Result<String, String> {
    fs::read_to_string(filename).map_err(|err| err.to_string())
}
