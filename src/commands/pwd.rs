use std::path::PathBuf;

pub fn pwd_handler(path: &PathBuf) {
      match path.to_str() {
        Some(path_str) => println!("{}", path_str),
        None => eprintln!("Error: Path invalid"),
    }
}
