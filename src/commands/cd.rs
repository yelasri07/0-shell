use std::{env, io::ErrorKind, path::PathBuf};

use crate::utils::get_current_dir;

pub fn cd_handler(args: Vec<String>, prev_path: PathBuf, current_path: PathBuf, home: String) -> (PathBuf, PathBuf) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return (prev_path, current_path)
    }

    let mut new_dir: PathBuf = PathBuf::from(args.join(" "));

    if new_dir.as_os_str().is_empty() || new_dir.as_os_str() == "--" {
        new_dir = PathBuf::from(home);
    }

    let p_path = get_current_dir();

    if new_dir.as_os_str() == "-" {
        if prev_path.as_os_str().is_empty() {
            eprintln!("cd: OLDPWD not set");
            return (PathBuf::new(), current_path);
        }
        println!("{}", prev_path.display());
        new_dir = prev_path.clone();
    }

    if let Err(e) = env::set_current_dir(new_dir) {
        match e.kind() {
            ErrorKind::NotFound => eprintln!("cd: No such file or directory"),
            ErrorKind::PermissionDenied => eprintln!("cd: Permission denied"),
            _ => eprintln!("{}", e),
        }
        return (prev_path, current_path);
    }

    let c_path = get_current_dir();

    (p_path, c_path)
}
