use crate::commands::*;
use std::{fs, path::Path};
pub fn mv_handler(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("mv: missing file operand");
        return;
    }
    if args.len() == 1 {
        println!("mv: missing destination file operand after '{}'", args[0]);
        return;
    }
    let destination = Path::new(&args[args.len() - 1]);
    if !destination.exists() && args.len() == 2 {
        let src = Path::new(&args[0]);
        if !src.exists() {
            eprintln!("mv: cannot stat '{:?}': No such file or directory", src);
            return;
        }
        if let Err(e) = fs::create_dir(destination) {
            eprintln!("mv: {e}");
        }
    }
    let metadata = fs::metadata(destination);
    match metadata {
        Ok(dest) => {
            if dest.is_dir() {
                let mut files: Vec<String> = vec![];
                for opt in args {
                    let file = Path::new(&opt);
                    if !file.exists() {
                        eprintln!("mv: cannot stat '{opt}': No such file or directory");
                        continue;
                    }
                    files.push(opt);
                }
                cp_handler(files.clone());
                files.insert(0, "-r".to_string());
                rm_handler(files[..files.len()-1].to_vec());
            } else {
                eprintln!("mv: target '{:?}' is not a directory", destination);
                return;
            }
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
