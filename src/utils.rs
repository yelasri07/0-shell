use std::io::{self, Write};

pub fn read_line(path: &str) -> String {
    print!("{}", path);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    input
}
