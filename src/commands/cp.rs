use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Cp {
    pub options: Vec<String>,
    pub target: String,
}

impl Cp {
    pub fn new() -> Self {
        Self {
            options: vec![],
            target: String::new(),
        }
    }

    pub fn exec(&self, content: String) -> io::Result<()> {
        let src = self.options[0].clone();
        match get_file_state(&src) {
            Ok(meta) => {
                if meta.is_file() {
                    let mut file = File::create(&self.target)?;
                    file.write(content.as_bytes())?;
                } else if meta.is_dir() {
                    let file_name = Path::new(&src)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    let new_path = Path::new(&self.target).join(file_name);
                    let mut file = File::create(&new_path)?;
                    file.write_all(content.as_bytes())?;
                }
            }
            Err(_) => {
                let mut file = File::create(&src)?;
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
    cp.target = args[args.len() - 1].clone();
    for i in 0..args.len() - 1 {
        cp.options.push(args[i].clone());
    }

    let metadata = get_file_state(&cp.target);
    if metadata.is_err() {
        eprintln!("cp: cannot stat '{}': No such file or directory", cp.target);
        return;
    }

    let target = metadata.unwrap();

    if target.is_file() {
        if cp.options.len() != 1 {
            eprintln!("cp: target '{}' is not a directory", cp.target);
            return;
        }
        match read_file(&cp.options[0]) {
            Ok(content) => {
                if let Err(err) = cp.exec(content) {
                    eprintln!("cp: error copying file: {}", err);
                }
            }
            Err(err) => eprintln!("cp: error reading '{}': {}", cp.options[0], err),
        }
    }

    if target.is_dir() {
        for opt in cp.options.iter() {
            let src_path = Path::new(opt);
            let dest_path = Path::new(&cp.target);
            println!("{:?} =>{:?}", dest_path, src_path);
            let new_src_dir = if src_path.is_dir() {
                dest_path.join(src_path.file_name().unwrap())
            } else {
                PathBuf::from(opt)
            };

            if let Err(err) = Cp::copy_dir_recursive(src_path, &new_src_dir) {
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
pub fn read_file(file: &String) -> io::Result<String> {
    fs::read_to_string(file)
}
