use std::{fs, path::{Path, PathBuf}};

pub fn mkdir_handler(args: Vec<String>, current_path: PathBuf) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
    }
    let curr_path = Path::new(&current_path);
    for path in args {
        if !curr_path.exists() && !(path.starts_with("./") && path.starts_with("../")) {
            eprintln!("mkdir: cannot create directory '{}': No such file or directory", path);
            continue;
        }
        if let Err(err) = fs::create_dir(path) {
            eprintln!("mkdir: {}", err);
        }
    }
}
