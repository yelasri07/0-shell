use std::fs;

pub fn mkdir_handler(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
    }
    for path in args {
        if let Err(err) = fs::create_dir(path) {
            eprintln!("mkdir: {}", err);
        }
    }
}
