use std::{env, io::ErrorKind, path::PathBuf};

use crate::utils::get_current_dir;

pub fn cd_handler(args: Vec<String>, prev_path: PathBuf, current_path: &mut PathBuf, home: String) -> (PathBuf, PathBuf) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return (prev_path, current_path.to_path_buf())
    }

    let mut new_dir: PathBuf = PathBuf::from(args.join(" "));

    if new_dir.as_os_str().is_empty() || new_dir.as_os_str() == "--" {
        new_dir = PathBuf::from(home);
    }

    let p_path = get_current_dir();

    if new_dir.as_os_str() == "-" {
        if prev_path.as_os_str().is_empty() {
            eprintln!("cd: OLDPWD not set");
            return (PathBuf::new(), current_path.to_path_buf());
        }
        println!("{}", prev_path.display());
        new_dir = prev_path.clone();
    }

    if let Err(e) = env::set_current_dir(new_dir.clone()) {
        match e.kind() {
            ErrorKind::NotFound => eprintln!("cd: No such file or directory"),
            ErrorKind::PermissionDenied => eprintln!("cd: Permission denied"),
            _ => eprintln!("{}", e),
        }
        return (prev_path, current_path.to_path_buf());
    }

    let mut c_path = get_current_dir();
    if c_path.as_os_str().is_empty() {
        eprintln!("cd: error retrieving current directory: getcwd: cannot access parent directories: No such file or directory");
        current_path.push("..");
        c_path.push(current_path);
    }

    if new_dir.is_relative() {
        new_dir = get_logical_path(&p_path.join(new_dir).display().to_string());
    }

    println!("{:?}", new_dir);

    (p_path, c_path)
}

fn get_logical_path(path: &str) -> PathBuf {
    let mut path_elements = path.split('/').collect::<Vec<&str>>();
    for i in 0..path_elements.len() {
        if path_elements[i] == ".." {
            let mut cp = i - 1;
            while cp > 0 {
                if path_elements[cp] == ".." {
                    cp -= 1;
                    continue;
                }
                path_elements[cp] = "..";
                break;
            }
        }
    }

    let mut new_path = PathBuf::new();
    for i in 0..path_elements.len() {
        if path_elements[i] != ".." {
            new_path = new_path.join(path_elements[i]);
        }
    }

    new_path
}