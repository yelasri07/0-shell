use std::{env, io::ErrorKind};

use crate::utils::get_current_dir;

pub fn cd_handler(args: Vec<String>, prev_path: &String) -> String {
    let mut new_dir: &str = &args.join(" ");

    if new_dir.is_empty() || new_dir == "~" {
        new_dir = "/home/";
    }

    let path = get_current_dir();

    if new_dir == "-" {
        if prev_path.is_empty() {
            eprintln!("cd: OLDPWD not set");
            return "".to_string();
        }
        new_dir = prev_path;
    }

    if let Err(e) = env::set_current_dir(new_dir) {
        match e.kind() {
            ErrorKind::NotFound => eprintln!("cd: No such file or directory"),
            ErrorKind::PermissionDenied => eprintln!("cd: Permission denied"),
            _ => eprintln!("{}", e),
        }
        return prev_path.to_string();
    }

    path
}
