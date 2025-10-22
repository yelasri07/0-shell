use std::{
    env, fs, io::{self, Write}, path::{Path, PathBuf}
};

use colored::Colorize;

pub fn read_line(path: &str, home: &str) -> (String,usize) {
    let p = if path.starts_with(home) {
        "~".to_string() + &path[home.len()..path.len()]
    } else {
        path.to_string()
    };

    print!("{} ", p.blue().underline().bold());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let n_bytes=io::stdin().read_line(&mut input).unwrap();
    
    (input.trim().to_string(),n_bytes)
}

pub fn get_current_dir() -> PathBuf {
    match env::current_dir() {
        Ok(path) => path,
        Err(_) => PathBuf::new()
    }
}

pub fn direct_children(dir: &Path) -> Vec<PathBuf> {
    let mut children = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            children.push(entry.path());
        }
    }
    children
}