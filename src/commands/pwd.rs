use std::env;

pub fn pwd_handler(_args: Vec<String>, path: &str) {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(_) => println!("{}", path),
    }
}
