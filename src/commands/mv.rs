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
        let src_meta = fs::metadata(src);
        match src_meta {
            Ok(file) => {
                if file.is_dir() {
                    if let Err(e) = fs::rename(src, destination) {
                        eprintln!("mv: {e}");
                    }
                } else if file.is_file() {
                    if let Err(e) = fs::rename(src, destination) {
                        eprintln!("mv: {e}");
                        return;
                    }
                }
            }
            Err(e) => {
                eprintln!("mv: {e}");
            }
        }
        return;
    }
    let metadata = fs::metadata(destination);
    match metadata {
        Ok(dest) => {
            if dest.is_dir() {
                let mut files = get_args(&args);
                cp_handler(files.clone());
                files.insert(0, "-r".to_string());
                rm_handler(files[..files.len() - 1].to_vec());
            } else if dest.is_file() {
                let files = get_args(&args);
                if files.len() > 2 {
                    eprintln!("mv: target '{:?}' is not a directory", destination);
                    return;
                } else if files.len() == 2 {
                    let src_meta = fs::metadata(&files[0]);
                    if let Ok(file) = src_meta {
                        if !file.is_file() {
                            eprintln!(
                                "mv: cannot overwrite non-directory {:?} with directory '{}'",
                                destination, files[0]
                            );
                            return;
                        }
                    }
                }
                cp_handler(files.clone());
                rm_handler(files[..files.len() - 1].to_vec());
            }
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}

pub fn get_args(args: &Vec<String>) -> Vec<String> {
    let mut files: Vec<String> = vec![];
    for opt in args {
        let file = Path::new(&opt);
        if !file.exists() {
            eprintln!("mv: cannot stat '{opt}': No such file or directory");
            continue;
        }
        files.push(opt.clone());
    }
    files
}
