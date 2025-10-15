use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Cp {
    pub options: Vec<String>, // fichiers sources
    pub target: String,       // destination
}

impl Cp {
    pub fn new() -> Self {
        Self {
            options: vec![],
            target: String::new(),
        }
    }

    pub fn exec(src_path: &Path, dest_path: &Path, content: String) -> io::Result<()> {
        // Vérifie le statut de la destination
        match fs::metadata(dest_path) {
            Ok(meta) => {
                if meta.is_file() {
                    // Destination est un fichier → on écrase
                    let mut file = File::create(dest_path)?;
                    file.write_all(content.as_bytes())?;
                } else if meta.is_dir() {
                    // Destination est un dossier → on copie le fichier dedans
                    let file_name = src_path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    let new_path = dest_path.join(file_name);
                    let mut file = File::create(&new_path)?;
                    file.write_all(content.as_bytes())?;
                }
            }
            Err(_) => {
                // Si la destination n'existe pas → créer un fichier
                let mut file = File::create(dest_path)?;
                file.write_all(content.as_bytes())?;
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
                // recursive call via Self::
                Self::copy_dir_recursive(&child, &dest_path)?;
            }
        }

        Ok(())
    }
}

pub fn cp_handler(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("cp: missing file operand");
        return;
    }

    let mut cp = Cp::new();
    cp.target = args.last().unwrap().clone();
    cp.options = args[..args.len() - 1].to_vec();

    let dest_meta = fs::metadata(&cp.target);

    // Si la destination n'existe pas
    if dest_meta.is_err() {
        // Si plusieurs sources → erreur
        if cp.options.len() > 1 {
            eprintln!("cp: target '{}' is not a directory", cp.target);
            return;
        }

        // Sinon on crée le fichier destination et on copie
        if let Ok(content) = read_file(&cp.options[0]) {
            let src_path = Path::new(&cp.options[0]);
            let dest_path = Path::new(&cp.target);
            if src_path == dest_path {
                eprintln!(
                    "cp: '{}' and '{}' are the same file",
                    cp.options[0], cp.target
                );
                return;
            }
            if let Err(err) = Cp::exec(src_path, dest_path, content) {
                eprintln!("cp: error copying file: {}", err);
            }
        } else {
            eprintln!("cp: cannot read file '{}'", cp.options[0]);
        }
        return;
    }

    let target = dest_meta.unwrap();

    if target.is_file() {
        if cp.options.len() != 1 {
            eprintln!("cp: target '{}' is not a directory", cp.target);
            return;
        }

        let src_path = Path::new(&cp.options[0]);
        let dest_path = Path::new(&cp.target);

        if let Ok(content) = read_file(&cp.options[0]) {
            if src_path == dest_path {
                eprintln!(
                    "cp: '{}' and '{}' are the same file",
                    cp.options[0], cp.target
                );
                return;
            }
            if let Err(err) = Cp::exec(src_path, dest_path, content) {
                eprintln!("cp: error copying file: {}", err);
            }
        } else {
            eprintln!("cp: cannot read file '{}'", cp.options[0]);
        }
    }

    if target.is_dir() {
        for opt in cp.options.iter() {
            let src_path = Path::new(opt);
            let dest_path = Path::new(&cp.target);
            let new_src_dir = if src_path.is_dir() {
                dest_path.join(src_path.file_name().unwrap())
            } else {
                if let Ok(content) = read_file(opt) {
                    if let Err(err) = Cp::exec(src_path, dest_path, content) {
                        eprintln!("cp: error copying file: {}", err);
                    }
                }
                PathBuf::from(opt)
            };

            if let Err(err) = Cp::copy_dir_recursive(src_path, &new_src_dir) {
                eprintln!("cp: cannot copy directory: {}", err);
                return;
            }
        }
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
pub fn read_file(file: &String) -> io::Result<String> {
    fs::read_to_string(file)
}
