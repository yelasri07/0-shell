use crate::utils::direct_children;
use std::{
    fs::{self, File},
    io::{self},
    os::unix::fs::FileTypeExt,
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

    pub fn exec(src_path: &Path, dest_path: &Path) -> io::Result<()> {
        match fs::metadata(dest_path) {
            Ok(meta) => {
                if meta.is_file() {
                    if let Err(err) = fs::copy(&src_path, dest_path) {
                        return Err(err);
                    }
                } else if meta.is_dir() {
                    let new_path = dest_path.join(src_path);

                    if let Err(err) = fs::copy(&src_path, new_path) {
                        return Err(err);
                    }
                } else {
                    if let Err(err) = fs::copy(&src_path, dest_path) {
                        return Err(err);
                    }
                }
            }
            Err(_) => {
                if let Err(err) = fs::copy(&src_path, dest_path) {
                    return Err(err);
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
    if args.len() < 2 {
        eprintln!("cp: missing file operand");
        return;
    }

    let mut cp = Cp::new();
    cp.target = args.last().unwrap().clone();
    cp.options = args[..args.len() - 1].to_vec();

    let dest_meta = fs::metadata(&cp.target);

    if dest_meta.is_err() && cp.options.len() == 1 {
        let src_path = Path::new(&cp.options[0]);
        let dest_path = Path::new(&cp.target);
        if src_path == dest_path {
            eprintln!(
                "cp: '{}' and '{}' are the same file",
                cp.options[0], cp.target
            );
            return;
        }
        if let Err(err) = Cp::exec(src_path, dest_path) {
            eprintln!("cp: error copying file: {}", err);
        }

        return;
    } else if dest_meta.is_err() && cp.options.len() != 1 {
        eprintln!("cp: target '{}' is not a directory", cp.target);
        return;
    }

    let target = dest_meta.unwrap();

    if target.file_type().is_file() || target.file_type().is_fifo() {
        if cp.options.len() != 1 {
            eprintln!("cp: target '{}' is not a directory", cp.target);
            return;
        }

        let src_path = Path::new(&cp.options[0]);
        let dest_path = Path::new(&cp.target);

        if src_path == dest_path {
            eprintln!(
                "cp: '{}' and '{}' are the same file",
                cp.options[0], cp.target
            );
            return;
        }
        if let Err(err) = Cp::exec(src_path, dest_path) {
            eprintln!("cp: error copying file: {}", err);
            return;
        }
    }

    if target.file_type().is_dir() {
        for opt in cp.options.iter() {
            let src_path = Path::new(opt);
            let dest_path = Path::new(&cp.target);
            let new_src_dir = if src_path.is_dir() {
                dest_path.join(src_path.file_name().unwrap())
            } else {
                if let Err(err) = Cp::exec(src_path, dest_path) {
                    eprintln!("cp: error copying file: {}", err);
                    return;
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
