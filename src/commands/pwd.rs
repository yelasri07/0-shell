use std::env;
use std::path::PathBuf;

pub fn pwd_handler(args: Vec<String>) {
    if !args.is_empty() {
        eprintln!("pwd: too many arguments");
        return;
    }

    let path: PathBuf = env::current_dir().unwrap();
    println!("{}", path.display());

}
