use std::{env, io::{self, Write}};

pub fn read_line(path: &str) -> String {
    print!("{}", path);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn get_current_dir() -> String {
    match env::current_dir() {
        Ok(path) => format!("{:?}", path).trim_matches('"').to_string(),
        Err(e) => {
            println!("{}", e);
            "".to_string()
        }
    }
}