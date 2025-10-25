use chrono::{DateTime, Local};
// use colored::Colorize;
use core::fmt;
use std::collections::HashSet;
use std::fmt::{Display, write};
use std::fs::{self};
use std::fs::{Metadata, symlink_metadata};
use std::io::{self, Error};
use std::os::unix::fs::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use users::{get_group_by_gid, get_user_by_uid};

pub fn ls_handler(args: Vec<String>, current_path: PathBuf) {
    let ls = match LsConfig::new(args, current_path) {
        Ok(ls) => ls,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!("{:?}", ls);

    // ls.execute();
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
struct Target(String, Entity);

#[derive(Debug, Default, Clone)]
struct Flags {
    long: bool,
    all: bool,
    classify: bool,
}

#[derive(Debug, Default, Clone)]
struct LsConfig {
    current_path: PathBuf,
    valid_flags: Vec<char>,
    flags: Flags,
    targets: Vec<Target>,
}

impl LsConfig {
    fn new(args: Vec<String>, current_path: PathBuf) -> Result<Self, String> {
        let valid_flags = ['l', 'a', 'F'].into_iter().collect();
        let mut ls = Self {
            flags: Flags {
                long: false,
                all: false,
                classify: false,
            },
            current_path,
            targets: Vec::new(),
            valid_flags,
        };

        let (flag_args, target_args) = args.into_iter().partition::<Vec<_>, _>(|arg| {
            arg.starts_with('-') && !arg.chars().skip(1).collect::<String>().is_empty()
        });
        ls.parse_flags(flag_args)?;
        ls.parse_targets(target_args);
        Ok(ls)
    }

    fn parse_flags(&mut self, args: Vec<String>) -> Result<(), String> {
        for arg in args {
            for ch in arg.chars().skip(1) {
                if !self.valid_flags.contains(&ch) {
                    return Err(format!(""));
                }
                match ch {
                    'l' => self.flags.long = true,
                    'a' => self.flags.all = true,
                    'F' => self.flags.classify = true,
                    _ => {} // Unreachable due to valid_flags check
                }
            }
        }
        Ok(())
    }
    fn absolute_path(&self, path: String) -> PathBuf {
        let path = if !path.starts_with("/") {
            format!("{}/{}", self.current_path.display(), path)
        } else {
            path.clone()
        };

        PathBuf::from(path)
    }

    fn parse_targets(&mut self, args: Vec<String>) {
        println!("targets =>> {:?}", args);
        if args.is_empty() {
            let current_dir = match Entity::new(None, self.current_path.clone()) {
                Ok(entity) => entity,
                Err(err) => {
                    eprintln!("Err listing current directory => {:?}", err.kind());
                    return;
                }
            };

            self.targets.push(Target(".".to_string(), current_dir));
            return;
        }

        for elem in args {
            let abs_path = self.absolute_path(elem.clone());

            let target_entity = match Entity::new(None, abs_path) {
                Ok(entity) => entity,
                Err(err) => {
                    eprintln!("Err listing current directory {:?}", err.kind());
                    return;
                }
            };

            let target = Target(elem, target_entity);
            self.targets.push(target);
        }

        // order targets by files then folders
        self.targets.sort_by(|a, b| {
            let is_a_dir = a.1.file_type == EntityType::Dir;
            let is_b_dir = b.1.file_type == EntityType::Dir;
            if is_a_dir != is_b_dir {
                return is_a_dir.cmp(&is_b_dir);
            } else {
                return a.0.cmp(&b.0);
            }
        });
    }

    // fn execute(&mut self) {
    //     for target in self.targets.clone() {
    //         let header_entity = Entity::new(None, Path::new(&target.1).to_path_buf());

    //         let mut list = List::new(target.0, self.flags.contains(&'l'));
    //         list.get_items(
    //             header_entity,
    //             self.flags.contains(&'l'),
    //             self.flags.contains(&'F'),
    //         );

    //         let list_len = list.items.len();
    //         for (index, file) in list.items.iter_mut().enumerate() {
    //             let mut sep = " ";

    //             if !self.flags.contains(&'a') && file.name.starts_with(".") {
    //                 continue;
    //             }

    //             file.is_classified = self.flags.contains(&'F');

    //             if self.flags.contains(&'l') {
    //                 file.long_list();
    //                 sep = "\n";
    //             }

    //             print!("{}", file);
    //             if index != list_len - 1 {
    //                 print!("{sep}")
    //             }
    //         }
    //     }
    // }
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

    fn get_items(&mut self, target: Entity, long_listing: bool, is_classify: bool) {
        let target_clone = target.clone();

        let read_directory = match target_clone.file_type {
            EntityType::Dir => true,
            EntityType::SymLink => {
                if long_listing && !self.header.ends_with("/") {
                    false
                } else {
                    true
                }
            }
            _ => false,
        };

        println!("is dir: {}", read_directory);

        if !read_directory {
            self.items.push(target.clone());
        } else {
            let files = fs::read_dir(target.path.clone())
                .unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()
                .unwrap();

            self.items = files
                .into_iter()
                .map(|p| Entity::new(Some(target.path.clone()), p).unwrap())
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
    minor: Option<u32>,
    major: Option<u32>,
    size: String,
    time: String,
    name: String,
    link_target: Option<PathBuf>,
    is_classified: bool,
    is_long: bool,
}

impl Entity {
    fn new(parent: Option<PathBuf>, path: PathBuf) -> Result<Self, Error> {
        let metadata = fs::symlink_metadata(path.clone())?;

        let mut new = Self {
            parent: parent,
            path: path.clone(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            ..Default::default()
        };

        new.file_type = get_file_type(metadata.permissions().mode());
        new.link_target = read_link(new.clone());
        Ok(new)
    }

    fn long_list(&mut self) {
        let metadata = fs::symlink_metadata(self.path.clone()).expect("");
        self.is_long = true;

        let permissions = get_permissions(metadata.permissions().mode() & 0o777);
        if permissions.contains("x") && self.file_type == EntityType::File {
            self.file_type = EntityType::Executable;
        }
        self.permissions = permissions;

        self.nlink = metadata.nlink();

        let uid = metadata.uid();
        let gid = metadata.gid();
        self.uid = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or(uid.to_string());
        self.gid = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or(gid.to_string());

        self.size = metadata.clone().len().to_string();
        self.get_modified_time(metadata);
        
        if let Some(data) = major_minor(self.clone()) {
            self.major = Some(data.0);
            self.minor = Some(data.1);
        } else {
            self.major = None;
            self.minor = None;
        }
    }

    fn get_modified_time(&mut self, metadata: Metadata) {
        let modified_time = metadata.modified().unwrap();
        let datetime: DateTime<Local> = modified_time.into();

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

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. Determine base name and suffix
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

        if self.is_long {
            let mode = format!("{}{}", symbol, self.permissions);

            writeln!(
                f,
                "{:10} {:>4} {:<8} {:<8} {:>8} {} {}{}",
                mode, self.nlink, self.uid, self.gid, self.size, self.time, name, sufix
            )
        } else {
            // Short listing: only name and suffix
            writeln!(f, "{}{}", self.name, sufix)
        }
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
    println!("read_link for [{:?}] of type: [{:?}]", entity.name, entity.file_type);
    let a = fs::read_link(entity.path);
    if entity.file_type != EntityType::SymLink {
        println!("not a symlink no need to follow it");
        return None;
    }

    if let Ok(l) = a {
        return Some(l);
    } else {
        return None;
    }
}
fn major_minor(entity: Entity) -> Option<(u32, u32)> {


    if entity.file_type != EntityType::CharacterDevice
        || entity.file_type != EntityType::BlockDevice
    {
        return None;
    }
    if let Ok(meta) = fs::metadata(entity.path.clone()) {
        let id_device = meta.rdev();
        let major = libc::major(id_device);
        let minor = libc::minor(id_device);
        return Some((major, minor));
    } else {
        None
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
    // let meta = fs::symlink_metadata(path)
    let path = Path::new(arg);
    path.exists()
}

fn is_dir(path: String) -> bool {
    let path = Path::new(&path);
    path.is_dir()
}
