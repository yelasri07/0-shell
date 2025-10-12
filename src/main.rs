mod shell;
mod commands;
mod read_line;

use read_line::*;

use shell::*;

fn main() {
    let mut shell = Shell::new();

    loop {
        let input: &str = &read_line("$ ");

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
