use std::{env, fs};
use std::error::Error;

pub fn get_input(name: &str) -> Result<String, Box<dyn Error>> {
    let cwd = env::current_dir()?;
    let input_path = cwd.join("input/").join(name);

    match fs::read_to_string(input_path) {
        Ok(x) => Ok(x),
        Err(x) => Err(Box::from(x))
    }
}

pub fn get_lines(name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let input = get_input(name)?;
    let lines = input.lines();
    Ok(lines.map(|s| s.to_string()).collect())
}
