mod shell;
mod commands;
mod utils;

use utils::*;

use shell::*;

fn main() {
    let mut shell = Shell::new();

    loop {
        let input: &str = &read_line("$ ");

        shell.set_cmd("".to_string());
        shell.set_args(vec![]);
        shell.set_arg("".to_string());
        shell.set_cmd_len(0);

        if let Err(e) = shell.parse_cmd(input) {
            println!("{}", e);
            continue;
        }

        let cmd: &str = &shell.get_cmd();
        let cmd_len = shell.get_cmd_len();

        shell.parse_args(&input[cmd.len() + cmd_len..input.len()]);

        shell.run();
    }
}
