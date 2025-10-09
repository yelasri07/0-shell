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

        if let Err(e) = parser.set_cmd(input) {
            println!("{}", e);
            continue;
        }

        let cmd = parser.get_cmd();

        parser.set_args(&input[cmd.len()..input.len()]);
    }
}
