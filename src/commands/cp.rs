use std::fs;
pub struct Cp {
    pub dir_flag: bool,
    pub cible: String,
    pub destination: Vec<String>,
}
impl Cp {
    pub fn new() -> Self {
        Self {
            dir_flag: false,
            cible: "".to_string(),
            destination: vec![],
        }
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
        cp.destination.push(file);
    }
    let meatdata = fs::metadata(cp.cible.clone());
    if let Err(_) = meatdata {
        eprintln!(
            "cp: cannot stat '{}': No such file or directory",
            cp.cible.clone()
        );
        return;
    }

    // let data = meatdata.unwrap();

    if let Ok(data) = meatdata {
        if data.is_dir() && !cp.dir_flag {
            println!(
                "cp: -r not specified; omitting directory '{}'",
                cp.cible.clone()
            );
        }
        return;
    }
}
