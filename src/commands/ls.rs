use chrono::{DateTime, Local};
use core::fmt;
use std::collections::HashSet;
use std::fmt::{Display, format};
use std::fs::Metadata;
use std::fs::{self, FileType};
use std::io;
use std::os::unix::fs::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use users::{get_group_by_gid, get_user_by_uid};
use colored::Colorize;

#[derive(Debug, Eq, PartialEq, Default, Clone)]
struct Target(String, String);

#[derive(Debug, Default, Clone)]
struct LsConfig {
    current_path: String,
    valid_flags: Vec<char>,
    flags: HashSet<char>,
    targets: Vec<Target>,
}

impl LsConfig {
    fn new(valid_flags: Vec<char>, current_path: String) -> Self {
        Self {
            current_path,
            valid_flags,
            ..Default::default()
        }
    }

    fn parse_flags(&mut self, args: Vec<String>) -> Result<(), String> {
        // println!("flags =>> {:}",args );
        for elem in args {
            for ch in elem.chars().skip(1) {
                if !self.valid_flags.contains(&ch) {
                    return Err(format!("ls: invalid option -- '{}'", ch));
                }
                self.flags.insert(ch);
            }
        }
        Ok(())
    }

    fn parse_targets(&mut self, args: Vec<String>) {
        if args.is_empty() {
            self.targets
                .push(Target(".".to_string(), self.current_path.clone()));
            return;
        }

        for elem in args {
            let full_path = if !elem.starts_with("/") {
                format!("{}/{}", self.current_path, elem)
            } else {
                elem.clone()
            };

            if !path_exists(&full_path) {
                eprintln!("ls: cannot access '{}': No such file or directory", elem);
                continue;
            }

            let target = Target(elem, full_path);
            self.targets.push(target);
        }

        // order targets by files then folders
        self.targets.sort_by(|a, b| {
            let is_a_dir = is_dir(a.clone().1);
            let is_b_dir = is_dir(b.clone().1);
            if is_a_dir != is_b_dir {
                return is_a_dir.cmp(&is_b_dir);
            } else {
                return a.0.cmp(&b.0);
            }
        });
    }

    fn targets_len(&self) -> usize {
        self.targets.len()
    }

    fn execute(&mut self) {
        for target in self.targets.clone() {
            let header_entity = Entity::new(Path::new(&target.1).to_path_buf());

            let mut list = List::new(target.0, self.flags.contains(&'l'));
        }
    }
}

#[derive(Debug, Default, Clone)]
struct List {
    header: String,
    total: usize,
    items: Vec<Entity>,
    sep: char,
}

impl List {
    fn new(header: String, long_listing: bool) -> Self {
        let sep = if long_listing { '\n' } else { ' ' };
        let new_list = Self {
            header,
            sep,
            ..Default::default()
        };

        new_list
    }

    fn get_items(&mut self, target: Entity, long_listing: bool) {
        let mut read_directory = false;
        let mut path_to_list = target.path;

        if let EntityType::Dir = target.file_type {
            read_directory = true;
        } else if let EntityType::Dir = target.file_type {
            let follow_link = long_listing && self.header.ends_with("/");
        };

        if !read_directory {
            // self.items
        }
    }
}

pub fn ls_handler(args: Vec<String>, current_path: String) {
    let valid_flags: Vec<char> = vec!['l', 'a', 'F'];
    let mut ls = LsConfig::new(valid_flags, current_path);

    // handle flags :
    let flags: Vec<String> = args
        .clone()
        .into_iter()
        .filter(|elem| elem.starts_with("-"))
        .collect();

    match ls.parse_flags(flags) {
        Err(message) => {
            eprintln!("{}", message);
            return;
        }
        Ok(_) => {}
    }

    // handle targets :
    let targets: Vec<String> = args
        .clone()
        .into_iter()
        .filter(|elem: &String| !elem.starts_with("-"))
        .collect();

    ls.parse_targets(targets);

    for (index, target) in ls.targets.iter().enumerate() {
        if is_dir(target.1.clone()) && ls.targets_len() > 1 {
            println!("{}:", target.0);
        }

        list_items(
            ls.flags.clone().into_iter().collect::<Vec<char>>(),
            target.1.to_owned(),
            target.0.to_owned(),
        );

        println!("");
        if index != ls.targets_len() - 1 {
            println!("");
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
enum EntityType {
    File,
    Dir,
    Executable,
    SymLink,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
    #[default]
    None,
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
struct Entity {
    parent: PathBuf,
    path: PathBuf,
    file_type: EntityType,
    permissions: String,
    nlink: u64,
    uid: String,
    gid: String,
    size: String,
    time: String,
    name: String,
    is_classified: bool,
    is_long: bool,
}

impl Entity {
    fn new(parent: PathBuf, path: PathBuf) -> Self {
        let mut new = Self {
            parent: parent,
            path: path.clone(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            ..Default::default()
        };

        let metadata = match fs::symlink_metadata(new.path.clone()) {
            Ok(res) => res,
            Err(_) => {
                eprintln!("ls: cannot access {:?}: Permission denied", new.name);
                return new;
            }
        };

        new.file_type = get_file_type(metadata.permissions().mode());
        new
    }

    fn long_list(&mut self) {
        let metadata = fs::symlink_metadata(self.path.clone()).expect("");
        self.is_long = true;
        self.get_permissions(metadata.clone());
        self.nlink = metadata.nlink();
        let uid = metadata.uid();
        let gid = metadata.gid();
        self.uid = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .expect("");
        self.gid = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .expect("");

        self.size = metadata.clone().len().to_string();
        self.get_modified_time(metadata);

        if self.file_type == EntityType::SymLink {
            self.name = format!("{} -> {}", self.name, read_link(self.clone()))
        }
    }

    fn get_permissions(&mut self, metadata: Metadata) {
        let permissions_bits = metadata.permissions().mode() & 0o777;
        let mut res = Vec::new();

        let masks: [u32; 9] = [
            0o400, 0o200, 0o100, 0o040, 0o020, 0o010, 0o004, 0o002, 0o001,
        ];

        for &mask in &masks {
            if permissions_bits & mask != 0 {
                match mask & 0o1 {
                    1 => {
                        res.push("x");
                        if self.file_type == EntityType::File {
                            self.file_type = EntityType::Executable;
                        }
                    }
                    _ => {
                        if mask & 0o4 != 0 {
                            res.push("r");
                        } else if mask & 0o2 != 0 {
                            res.push("w");
                        } else {
                            res.push("-");
                        }
                    }
                }
            } else {
                res.push("-");
            }
        }

        self.permissions = res.join("");
    }

    fn get_modified_time(&mut self, metadata: Metadata) {
        let modified_time = metadata.modified().unwrap();
        let datetime: DateTime<Local> = modified_time.into();

        let now = SystemTime::now()
            .duration_since(modified_time)
            .unwrap_or_default();
        let now = SystemTime::now()
            .duration_since(modified_time)
            .unwrap_or_default();
        let six_months = Duration::from_secs(60 * 60 * 24 * 30 * 6);

        self.time = if now > six_months {
            datetime.format("%b %e %Y").to_string()
        } else {
            datetime.format("%b %e %H:%M").to_string()
        };
    }
}

fn list_items(flags: Vec<char>, full_path: String, entity: String) {
    let mut list: Vec<Entity> = Vec::new();
    let get_path = |arg: String| {return Path::new(&arg).to_path_buf()};

    let parent = get_path(full_path.clone());
    if !is_dir(full_path.clone()) {
        let file = get_path(full_path.clone());
        list.push(Entity::new(parent.clone(),file));
    } else {
        // todo : handle the errors alhmar
        // todo : handle the errors alhmar
        let files = fs::read_dir(&full_path)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        list = files.into_iter().map(|p| Entity::new(parent.clone(),p)).collect();
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

        file.is_classified = flags.contains(&'F');

        if flags.contains(&'l') {
            file.long_list();
            sep = "\n";
        }

        print!("{}", file);
        if index != list_len - 1 {
            print!("{sep}")
        }
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (symbol, mut sufix) = match self.file_type {
            EntityType::File => ("-", ""),
            EntityType::Dir => ("d", "/"),
            EntityType::SymLink => ("l", "@"),
            EntityType::CharacterDevice => ("c", ""),
            EntityType::BlockDevice => ("b", ""),
            EntityType::Fifo => ("p", "|"),
            EntityType::Socket => ("s", "="),
            EntityType::Executable => ("-", "*"),
            EntityType::None => ("", ""),
        };

        if !self.is_classified {
            sufix = "";
        }

        let mut res = if self.is_long {
            format!(
                "{}{} {} {} {} {} {} {}{}",
                symbol,
                self.permissions,
                self.nlink,
                self.uid,
                self.gid,
                self.size,
                self.time,
                self.name,
                sufix
            )
        } else {
            format!("{}{}", self.name, sufix)
        };

        res = res.split_whitespace().collect::<Vec<_>>().join(" ");

        write!(f, "{}", res.trim())
    }
}

// utitlies :

fn read_link(entity: Entity) -> String {
    let a = fs::read_link(entity.path);
    let mut result = String::new();
    if let Ok(l) = a {
        // let link_target = if l.display().to_string()
        if l.exists() {
            result = format!("{} -> {}", entity.name, l.display().to_string().blue().bold())
        } else {
            result = format!("{} -> {}", entity.name, l.display().to_string().red().bold())
        }
    }
    result
}

fn get_file_type(mode: u32) -> EntityType {
    match mode & 0o170000 {
        0o100000 => EntityType::File,
        0o040000 => EntityType::Dir,
        0o120000 => EntityType::SymLink,
        0o020000 => EntityType::CharacterDevice,
        0o060000 => EntityType::BlockDevice,
        0o010000 => EntityType::Fifo,
        0o140000 => EntityType::Socket,
        _ => EntityType::None,
    }
}

fn path_exists(arg: &String) -> bool {
    let path = Path::new(arg);
    path.exists()
}

fn is_dir(path: String) -> bool {
    let path = Path::new(&path);
    path.is_dir()
}
