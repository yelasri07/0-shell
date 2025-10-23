use chrono::{DateTime, Local};
use colored::Colorize;
use core::fmt;
use std::collections::HashSet;
use std::fmt::{Display, format};
use std::fs::{self, FileType};
use std::fs::{Metadata, read};
use std::io;
use std::os::unix::fs::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use users::{get_group_by_gid, get_user_by_uid};

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

    
    ls.execute();
}




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

    fn execute(&mut self) {
        for target in self.targets.clone() {
            let header_entity = Entity::new(None, Path::new(&target.1).to_path_buf());

            let mut list = List::new(target.0, self.flags.contains(&'l'));
            list.get_items(header_entity, self.flags.contains(&'l'));

            let list_len = list.items.len();
            for (index, file) in list.items.iter_mut().enumerate() {
                let mut sep = " ";

                if !self.flags.contains(&'a') && file.name.starts_with(".") {
                    continue;
                }

                file.is_classified = self.flags.contains(&'F');

                if self.flags.contains(&'l') {
                    file.long_list();
                    sep = "\n";
                }

                print!("{}", file);
                if index != list_len - 1 {
                    print!("{sep}")
                }
            }
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
        let target_clone = target.clone();
        let mut path_to_list = target_clone.path;

        let read_directory = match target_clone.file_type {
            EntityType::Dir => true,
            EntityType::SymLink => {
                if long_listing && !self.header.ends_with("/") {
                    false
                } else {
                    path_to_list = read_link(target.clone()).unwrap();
                    true
                }
            }
            _ => false,
        };

        if !read_directory {
            self.items.push(target.clone());
        } else {
            let files = fs::read_dir(path_to_list)
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
                .unwrap();

            self.items = files
                .into_iter()
                .map(|p| Entity::new(Some(target.path.clone()), p))
                .collect();
        }

        self.items.sort_by(|a, b| {
            let file_a = a.name.strip_prefix(".").unwrap_or(&a.name);
            let file_b = b.name.strip_prefix(".").unwrap_or(&b.name);
            file_a.cmp(&file_b)
        });
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
    parent: Option<PathBuf>,
    path: PathBuf,
    file_type: EntityType,
    permissions: String,
    nlink: u64,
    uid: String,
    gid: String,
    size: String,
    time: String,
    name: String,
    link_target: Option<PathBuf>,
    is_classified: bool,
    is_long: bool,
}

impl Entity {
    fn new(parent: Option<PathBuf>, path: PathBuf) -> Self {
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
        self.permissions = get_permissions(metadata.permissions().mode() & 0o777);
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

        self.link_target = read_link(self.clone());
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

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut name = self.name.to_owned();
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

        if let Some(link_target) = self.link_target.clone() {
            name = format!("{} -> {}", self.name, link_target.display());
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
                name,
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

fn get_permissions(permissions_bits: u32) -> String {
    let mut res = Vec::new();
    let masks: [(u32, char); 9] = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    for &(mask, permission_char) in &masks {
        if permissions_bits & mask != 0 {
            res.push(permission_char);
        } else {
            res.push('-');
        }
    }

    res.iter().collect::<String>()
}

fn read_link(entity: Entity) -> Option<PathBuf> {
    let a = fs::read_link(entity.path);
    if entity.file_type != EntityType::SymLink {
        return None;
    }

    if let Ok(l) = a {
        return Some(l);
    } else {
        return None;
    }
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
