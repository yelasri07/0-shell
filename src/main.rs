mod commands;
mod shell;
mod utils;

use std::env;

use utils::*;

use shell::*;

fn main() {
    let mut shell = Shell::new();

    shell.set_current_path(get_current_dir());
    shell.set_home(env::var("HOME").unwrap_or("/home/".to_string()));
    loop {
        let (input, n_bytes) = read_line(&(shell.current_path.display().to_string() + "$"));
        if n_bytes == 0 {
            println!();
            break;
        }

        shell.set_args(vec![]);
        shell.set_arg("".to_string());
        shell.set_quotes_type('"');

        shell.parse_input(input.as_str());

        shell.run();
    }
}
