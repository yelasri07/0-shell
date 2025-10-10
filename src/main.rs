mod parsing;
use std::io::{self, Write};

use parsing::*;

fn main() {
    let mut parser = Parsing::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        parser.set_cmd("".to_string());
        parser.set_args(vec![]);

        if let Err(e) = parser.parse_cmd(input) {
            println!("{}", e);
            continue;
        }

        let cmd = parser.get_cmd();

        parser.parse_args(&input[cmd.len() + 1..input.len()]);

        let args = parser.get_args();

        println!("{:?}", args);
    }
}
