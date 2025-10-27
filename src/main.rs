mod commands;
mod shell;
mod utils;

use std::{env, io::{self, IsTerminal}, path::PathBuf};

use utils::*;

use shell::*;

use crate::commands::exit_handler;

fn main() -> Result<(), String> {
    let mut shell = Shell::new();
    if !io::stdout().is_terminal() {
        return Err(String::from("Broken pipe."));
    }

    shell.set_current_path(get_current_dir());
    if shell.current_path.as_os_str().is_empty() {
        shell.set_current_path(PathBuf::from("/"));
    }
    
    shell.set_home(env::var("HOME").unwrap_or("/home/".to_string()));
    loop {
        let (input, n_bytes) = read_line(
            &(shell.current_path.display().to_string() + "$"),
            &shell.home,
        );
        if n_bytes == 0 {
            println!();
            exit_handler();
        }

        shell.set_args(vec![]);
        shell.set_arg("".to_string());
        shell.set_quotes_type('"');

        if let Err(e) = shell.parse_input(input.as_str()) {
            eprintln!("{}", e);
            continue;
        }

        shell.run();
    }
}
