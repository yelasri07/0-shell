use crate::utils::direct_children;
use std::{fs, io::Error, path::Path};
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

        if src.is_dir() {
            if let Err(e) = fs::rename(src, destination) {
                eprintln!("mv: {e}");
            }
        } else if src.is_file() {
            if let Err(e) = fs::rename(src, destination) {
                eprintln!("mv: {e}");
                return;
            }
        }

        return;
    } else if !destination.exists() && args.len() > 2 {
        eprintln!("mv: target '{}' is not a directory", &args[args.len() - 1]);
        return;
    }

    if destination.is_dir() {
        for opt in args[..args.len() - 1].iter() {
            if opt == "." || opt == ".." {
                eprintln!(
                    "mv: cannot move '{opt}' to {:?}: Device or resource busy",
                    destination
                );
                continue;
            }
            let src: &Path = Path::new(&opt);
            if !src.exists() {
                eprintln!("mv: cannot stat '{opt}': No such file or directory");
                continue;
            }
            if src.is_file() {
                if let Some(file_name) = src.file_name() {
                    let new_dest = destination.join(file_name);

                    if let Err(e) = fs::rename(&src, new_dest) {
                        eprintln!("mv: {e}");
                        continue;
                    }
                } else {
                    eprintln!(
                        "mv: cannot join {:?} with {:?}",
                        destination,
                        src.file_name()
                    );
                    continue;
                }
            } else if src.is_dir() {
                if let Err(e) = move_dir_recursivly(src, destination) {
                    eprintln!("mv: {e}");
                    continue;
                }
            } else {
                if let Some(file_name) = src.file_name() {
                    let new_dest = destination.join(file_name);

                    if let Err(e) = fs::rename(&src, new_dest) {
                        eprintln!("mv: {e}");
                        continue;
                    }
                } else {
                    eprintln!(
                        "mv: cannot join {:?} with {:?}",
                        destination,
                        src.file_name()
                    );
                    continue;
                }
            }
        }
    } else if destination.is_file() {
        if args.len() > 2 {
            eprintln!("mv: target '{:?}' is not a directory", destination);
            return;
        } else if args.len() == 2 {
            let src_meta = fs::metadata(&args[0]);
            if let Ok(file) = src_meta {
                if !file.is_file() {
                    eprintln!(
                        "mv: cannot overwrite non-directory {:?} with directory '{}'",
                        destination, args[0]
                    );
                    return;
                }
            }
            let src_path = Path::new(&args[0]);
            if src_path.file_name() == destination.file_name() {
                eprintln!(
                    "mv: {:?} and {:?} are the same file",
                    src_path.file_name().unwrap(),
                    destination.file_name().unwrap()
                );
                return;
            }
            if let Err(e) = fs::rename(src_path, destination) {
                eprintln!("mv: {e}");
                return;
            }
        }
    }
}

pub fn move_dir_recursivly(src: &Path, dest: &Path) -> Result<(), Error> {
    let new_dest = dest.join(src.file_name().unwrap());
    if src.is_file() {
        if let Err(e) = fs::rename(src, new_dest) {
            return Err(e);
        }
    } else if src.is_dir() {
        if let Err(e) = fs::create_dir_all(&new_dest) {
            return Err(e);
        }
        for child in direct_children(src) {
            if let Err(e) = move_dir_recursivly(&child, &new_dest) {
                return Err(e);
            }
        }
        if let Err(e) = fs::remove_dir_all(&src) {
            return Err(e);
        }
    }

    Ok(())
}
