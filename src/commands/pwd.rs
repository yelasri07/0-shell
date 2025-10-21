use std::path::PathBuf;

pub fn pwd_handler(path: &PathBuf) {
    println!("{}", path.display())
}
