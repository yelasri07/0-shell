use std::env;

pub fn pwd_handler(_args: Vec<String>) {
    // type env::current_dir() : Result<PathBuf, std::io::Error>
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("Failed to get current directory :  "{}"", e),
    }
}
