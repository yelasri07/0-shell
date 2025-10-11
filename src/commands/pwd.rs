use std::env;

pub fn pwd_handler(_args: Vec<String>) {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("Erreur lors de la récupération du répertoire courant : {}", e),
    }
}
