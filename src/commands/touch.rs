use std::fs::OpenOptions;
use std::io;

pub fn touch_handler(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("touch: missing file operand");
        return;
    }

    for file_path in args {
        if let Err(e) = touch_file(&file_path) {
            eprintln!("touch: cannot touch '{}': {}", file_path, e);
        }
    }
}

fn touch_file(path: &str) -> io::Result<()> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;
    Ok(())
}