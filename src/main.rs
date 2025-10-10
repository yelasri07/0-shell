mod shell;
use std::io::{self, Write};
mod commands;

use shell::*;

fn main() {
    let mut shell = Shell::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        shell.set_cmd("".to_string());
        shell.set_args(vec![]);
        shell.set_arg("".to_string());

        if let Err(e) = shell.parse_cmd(input) {
            println!("{}", e);
            continue;
        }

        let cmd: &str = &shell.get_cmd();

        shell.parse_args(&input[cmd.len()..input.len()]);

        shell.run();
    }
}
