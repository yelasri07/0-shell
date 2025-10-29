use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use filetime::{FileTime, set_file_times};

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
     let path_obj = Path::new(path);
    
    if path_obj.exists() {
        // change time 
        let now = FileTime::now();
        set_file_times(path, now, now)?;
    } else {
        // create new file
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)?;
    }
    Ok(())
}