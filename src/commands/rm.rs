use std::fs;
use std::path::Path;

pub fn rm_handler(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("Usage: rm [-r] <file_or_directory>");
        return;
    }

    let mut dir_flag = false;
    let mut targets: Vec<String> = Vec::new();

    //check -r flage
    for arg in args {
        if arg == "-r" {
            dir_flag = true;
        } else if arg == "." || arg == ".." {
            eprintln!("rm: refusing to remove '.' or '..' directory: skipping '..'");
            continue;
        } else {
            targets.push(arg);
        }
    }

    if targets.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    for target in targets {
        let path = Path::new(&target);

        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", target);
            continue;
        }

        if path.is_file() {
            match fs::remove_file(path) {
                Ok(_) => (),
                Err(e) => eprintln!("rm: failed to remove '{}': {}", target, e),
            }
        } else if path.is_dir() {
            if dir_flag {
                match fs::remove_dir_all(path) {
                    Ok(_) => (),
                    Err(e) => eprintln!("rm: failed to remove directory '{}': {}", target, e),
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", target);
            }
        }
    }
}
