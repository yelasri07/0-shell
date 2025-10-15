use std::fs;
use std::path::Path;

pub fn rm_handler(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("Usage: rm <filename>");
        return;
    }

    for filename in &args {
        let path = Path::new(filename);
        //println!("rrrrrrrrrrrrr {}",path.display());

        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", filename);
            continue;
        }

        if path.is_file() {
            match fs::remove_file(path) {
                Ok(_) => (),
                Err(e) => eprintln!("rm: failed to remove '{}': {}", filename, e),
            }
        } else {
            eprintln!("rm: cannot remove '{}': Is a directory", filename);
        }
    }
}
