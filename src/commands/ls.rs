use core::fmt;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::fs::Metadata;
use std::io;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::path::{Path, PathBuf};

pub fn ls_handler(args: Vec<String>, current_path: String) {
    let valid_flags: Vec<char> = vec!['l', 'a', 'F'];

    let mut flags = HashSet::new();
    let mut entities = Vec::new();

    for arg in args {
        if !arg.starts_with('-') {
            entities.push(arg);
            continue;
        }

        for ch in arg.chars().skip(1) {
            if !valid_flags.contains(&ch) {
                eprintln!("ls: invalid option -- '{}'", ch);
                break;
            }
            flags.insert(ch);
        }
    }

    if entities.is_empty() {
        entities.push(current_path.clone());
    }

    entities.sort();
    let flags_vec: Vec<char> = flags.clone().into_iter().collect();

    for (index, entity) in entities.iter().enumerate() {
        let full_path = if !entity.starts_with("/") {
            format!("{}/{}", current_path, entity)
        } else {
            entity.to_string()
        };

        if is_dir(full_path.clone()) && entities.len() > 1 {
            println!("{}:", entity);
        }
        list_items(flags_vec.clone(), full_path, entity.to_string());

        println!("");
        if index != entities.len() - 1 {
            println!("");
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
enum EntityType {
    #[default]
    File,
    Dir,
    Executable,
    SymLink,
    Fifo,
    Socket,
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
struct Entity {
    path: PathBuf,
    file_type: EntityType,
    permissions: String,
    uid: String,
    gid: String,
    size: String,
    time: String,
    name: String,
}

impl Entity {
    fn new(path: PathBuf) -> Self {
        Self {
            path: path.clone(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            ..Default::default()
        }
    }

    fn classify(&mut self, _is_long_listing: bool) {
        todo!();
    }

    fn long_list(&mut self) {
        let metadata = self.metadata();
        self.file_type(metadata.clone());
        let permissions_bits = format!("{:o}", metadata.permissions().mode() & 0o777);
        let file_type_bits = format!("{:o}", metadata.permissions().mode() & 0o170000);
        let file_type_bits2 = format!("{:o}", metadata.permissions().mode() & 0o770000);
        
        println!("mode: {:o}", metadata.permissions().mode());
        println!("file type bits: {} ", file_type_bits);
        println!("file type bits: {} ", file_type_bits2);
        println!("permissions bits: {} ", permissions_bits);
    }

    fn file_type(&mut self, metadata: Metadata) {
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            self.file_type = EntityType::Dir;
        } else if file_type.is_symlink() {
            self.file_type = EntityType::SymLink;
        } else if file_type.is_fifo() {
            self.file_type = EntityType::Fifo;
        } else if file_type.is_socket() {
            self.file_type = EntityType::Socket;
        } else if file_type.is_fifo() {
            self.file_type = EntityType::File;
        }
    }

    fn metadata(&self) -> Metadata {
        fs::symlink_metadata(self.path.clone()).expect("REASON")
    }
}

fn list_items(flags: Vec<char>, full_path: String, entity: String) {
    let mut list: Vec<Entity> = Vec::new();

    if !path_exists(full_path.clone()) {
        eprintln!("ls: cannot access '{}': No such file or directory", entity);
        return;
    }

    if !is_dir(full_path.clone()) {
        let file = Path::new(&full_path).to_path_buf();
        list.push(Entity::new(file));
    } else {
        let files = fs::read_dir(&full_path)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        list = files.into_iter().map(|p| Entity::new(p)).collect();
    }
    let list_len = list.len();
    list.sort_by(|a, b| {
        let file_a = a.name.strip_prefix(".").unwrap_or(&a.name);
        let file_b = b.name.strip_prefix(".").unwrap_or(&b.name);
        file_a.cmp(&file_b)
    });
    for (index, file) in list.iter_mut().enumerate() {
        let mut sep = " ";

        if !flags.contains(&'a') && file.name.starts_with(".") {
            continue;
        }

        if flags.contains(&'F') {
            // file.classify();
        }

        if flags.contains(&'l') {
            file.long_list();
            sep = "\n";
            // println!("apply long listing");
        }

        print!("{}", file);
        if index != list_len - 1 {
            print!("{sep}")
        }
    }
}

// utitlies :

fn path_exists(arg: String) -> bool {
    let path = Path::new(&arg);
    path.exists()
}

fn is_dir(path: String) -> bool {
    let path = Path::new(&path);
    path.is_dir()
}

fn format_permissions(_permissions: String) -> String {
    todo!();
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r#type = "";
        let res = format!(
            "{}{} {} {} {} {} {}",
            r#type, self.permissions, self.uid, self.gid, self.size, self.time, self.name
        );
        write!(f, "{}", res.trim())
    }
}
