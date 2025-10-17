use std::{fs, path::Path};

pub fn rm_handler(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    let mut dir_flag = false;
    for option in args {
        if option.chars().next().unwrap() == '-' && option == "-r" {
            dir_flag = true;
            continue;
        }
        let src = Path::new(&option);
        let meta_data = fs::metadata(src);
        match meta_data {
            Ok(data) => {
                if data.is_file() {
                    if let Err(e) = fs::remove_file(src) {
                        eprintln!("rm: {}", e);
                        continue;
                    }
                }
                if data.is_dir() && dir_flag {
                    if let Err(e) = fs::remove_dir_all(src) {
                        eprintln!("rm: {}", e);
                        continue;
                    }
                } else if data.is_dir() && !dir_flag {
                    eprintln!("rm: cannot remove '{}': Is a directory", option);
                    continue;
                }
            }
            Err(err) => {
                eprintln!("rm: {}", err);
                continue;
            }
        }
    }
}
