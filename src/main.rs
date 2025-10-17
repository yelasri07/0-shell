mod commands;
mod shell;
mod utils;

use utils::*;

use shell::*;

fn main() {
    let mut shell = Shell::new();

    loop {
        shell.set_current_path(get_current_dir());

        let (input, n_bytes) = read_line(&(shell.current_path.to_string() + "$"));
        if n_bytes == 0 {
            println!();
            break;
        }

        shell.set_args(vec![]);
        shell.set_arg("".to_string());

        shell.parse_input(input.as_str());

        // println!("{:?}", shell.args);

        shell.run();
    }
}
