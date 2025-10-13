use std::{fs, io::Error};
#[derive(Debug, Clone)]
pub struct Cp {
    pub dir_flag: bool,
    pub cible: String,
    pub destinations: Vec<String>,
}
impl Cp {
    pub fn new() -> Self {
        Self {
            dir_flag: false,
            cible: "".to_string(),
            destinations: vec![],
        }
    }
    pub fn read_file(&self) -> Result<String, Error> {
        fs::read_to_string(&self.cible)
    }

    pub fn exec(&self, content: String) -> Option<()> {
        for dist in self.destinations.clone() {
            println!("{dist}");
        }
        Some(())
    }
}
pub fn cp_handler(args: Vec<String>) {
    if args.is_empty() {
        return;
    }
    let mut cp = Cp::new();
    for file in args {
        if file.chars().next() == Some('-') && file == "-r" && !cp.dir_flag {
            cp.dir_flag = true;
            continue;
        } else if file.chars().next() == Some('-') && file != "-r" {
            return;
        }
        if cp.cible.is_empty() {
            cp.cible = file;
            continue;
        }
        cp.destinations.push(file);
    }
    let meatdata= get_file_state(cp.cible.clone()) ;
    if let Err(_) = meatdata {
        eprintln!(
            "cp: cannot stat '{}': No such file or directory",
            cp.cible.clone()
        );
        return;
    }

    if let Ok(data) = meatdata {
        if data.is_dir() && !cp.dir_flag {
            println!(
                "cp: -r not specified; omitting directory '{}'",
                cp.cible.clone()
            );
            return;
        }
        if data.is_file() {
            let content = match cp.read_file() {
                Ok(v) => v,
                Err(_) => {
                    return;
                }
            };
            cp.exec(content);
        }
    }
}
    pub fn get_file_state(dist:String)->Result<fs::Metadata, Error> {
        fs::metadata(dist)
    }