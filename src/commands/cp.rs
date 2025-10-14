use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{
    fs::{self, File},
};

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
            cible: String::new(),
            destinations: vec![],
        }
    }

    pub fn read_file(&self) -> io::Result<String> {
        fs::read_to_string(&self.cible)
    }

    pub fn exec(&self, content: String) -> io::Result<()> {
        for dest in &self.destinations {
            match get_file_state(dest) {
                Ok(meta) => {
                    if meta.is_file() {
                        let mut file = File::create(dest)?;
                        file.write_all(content.as_bytes())?;
                    } else if meta.is_dir() {
                        let file_name = Path::new(&self.cible)
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "unknown".to_string());
                        let new_path = Path::new(dest).join(file_name);
                        let mut file = File::create(&new_path)?;
                        file.write_all(content.as_bytes())?;
                    }
                }
                Err(_) => {
                    // destination doesn’t exist → try to create it as a file
                    let mut file = File::create(dest)?;
                    file.write_all(content.as_bytes())?;
                }
            }
        }
        Ok(())
    }

    pub fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        for child in direct_children(src) {
            let file_name = match child.file_name() {
                Some(name) => name,
                None => continue,
            };
            let dest_path = dest.join(file_name);

            let meta = fs::metadata(&child)?;
            if meta.is_file() {
                let mut input = File::open(&child)?;
                let mut output = File::create(&dest_path)?;
                io::copy(&mut input, &mut output)?;
            } else if meta.is_dir() {
                
                Self::copy_dir_recursive(&child, &dest_path)?;
            }
        }

        Ok(())
    }
}

pub fn cp_handler(args: Vec<String>) {
    if args.is_empty() {
        return;
    }

    let mut cp = Cp::new();

    for file in args {
        if file == "-r" && !cp.dir_flag {
            cp.dir_flag = true;
            continue;
        } else if file.starts_with('-') && file != "-r" {
            eprintln!("cp: invalid option '{}'", file);
            return;
        }

        if cp.cible.is_empty() {
            cp.cible = file;
            continue;
        }

        cp.destinations.push(file);
    }

    let metadata = get_file_state(&cp.cible);
    if metadata.is_err() {
        eprintln!("cp: cannot stat '{}': No such file or directory", cp.cible);
        return;
    }

    let data = metadata.unwrap();

    if data.is_dir() && !cp.dir_flag {
        eprintln!("cp: -r not specified; omitting directory '{}'", cp.cible);
        return;
    }

    if data.is_file() {
        match cp.read_file() {
            Ok(content) => {
                if let Err(err) = cp.exec(content) {
                    eprintln!("cp: error copying file: {}", err);
                }
            }
            Err(err) => eprintln!("cp: error reading '{}': {}", cp.cible, err),
        }
    }

    if data.is_dir() && cp.dir_flag {
        for dest in &cp.destinations {
            let dest_path = Path::new(dest);
            let src_path = Path::new(&cp.cible);

            let new_dest_dir = if dest_path.is_dir() {
                dest_path.join(src_path.file_name().unwrap())
            } else {
                PathBuf::from(dest)
            };

            if let Err(err) = Cp::copy_dir_recursive(src_path, &new_dest_dir) {
                eprintln!("cp: cannot copy directory: {}", err);
                return;
            }
        }
    }
}

pub fn get_file_state<P: AsRef<Path>>(path: P) -> io::Result<fs::Metadata> {
    fs::metadata(path)
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
