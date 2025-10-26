use chrono::{DateTime, Local};
// use colored::Colorize;
use core::fmt;
use std::fs::{self};
use std::fs::{Metadata};
use std::io::{ Error, ErrorKind};
use std::os::unix::fs::*;
use std::path::{PathBuf};
use std::time::{Duration, SystemTime};
use users::{get_group_by_gid, get_user_by_uid};

pub fn ls_handler(args: Vec<String>, current_path: PathBuf) {
    let mut ls = match LsConfig::new(args, current_path) {
        Ok(ls) => ls,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    ls.execute();
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

        let mut flag_args: Vec<String> = Vec::new();
        let mut target_args: Vec<String> = Vec::new();
        let mut flag_as_target = false;

        for arg in args {
            if arg == "--" {
                flag_as_target = true;
                continue;
            }

            if arg.starts_with("-") && !flag_as_target {
                flag_args.push(arg);
            } else {
                target_args.push(arg);
            }
        }

        ls.parse_flags(flag_args)?;
        ls.parse_targets(target_args);
        Ok(ls)
    }

    fn parse_flags(&mut self, args: Vec<String>) -> Result<(), String> {
        for arg in args {
            for ch in arg.chars().skip(1) {
                if !self.valid_flags.contains(&ch) {
                    return Err(format!("ls: invalid option -- '{}'", ch));
                }
                match ch {
                    'l' => self.flags.long = true,
                    'a' => self.flags.all = true,
                    'F' => self.flags.classify = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn absolute_path(&self, path: String) -> PathBuf {
        let path = if !path.starts_with("/") {
            format!("{}/{}", self.current_path.display().to_string(), path)
        } else {
            path.clone()
        };

        PathBuf::from(path)
    }

    fn parse_targets(&mut self, args: Vec<String>) {
        if args.is_empty() {
            let current_dir = match Entity::new(self.current_path.clone()) {
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
            match Entity::new(abs_path) {
                Ok(entity) => {
                    let target = Target(elem, entity);
                    self.targets.push(target);
                }
                Err(err) => {
                    // println!("error while parsing: {}", )
                    handle_ls_erros(err, elem);
                }
            };
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

    fn execute(&mut self) {
        for mut target in self.targets.clone() {
            let mut list = List::new(target.0.clone());
            list.get_items(&mut target.1, self.flags.clone());

            if self.targets.len() > 1 {
                println!("{}:", list.header);
            }

            if self.flags.long {
                println!("total {}", list.total);
            }

            for (_, file) in list.items.iter_mut().enumerate() {

                if !self.flags.all && file.name.starts_with(".") {
                    continue;
                }

                file.is_classified = self.flags.classify;

                if self.flags.long {
                    file.long_list();
                }

                print!("{}", file);
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct List {
    header: String,
    total: u64,
    items: Vec<Entity>,
    // has_minor_major: bool,
}

impl List {
    fn new(header: String) -> Self {
        let new_list = Self {
            header,
            ..Default::default()
        };

        new_list
    }

    fn get_items(&mut self, target: &mut Entity, flags: Flags) {
        let mut is_dir = false;

        match target.file_type {
            EntityType::Dir => {
                is_dir = true;
            }
            _ => {}
        };

        if !is_dir && false {
            self.items.push(target.clone());
            return;
        } else {
            let files = match read_dir(target.path.clone()) {
                Ok(res) => res,
                Err(error) => {
                    println!("==> {:?}", error);
                    return;
                }
            };

            if flags.all {
                target.name = ".".to_string();
                self.items.push(target.clone());
                self.total += target.blocks;
                match Entity::new(target.parent.clone()) {
                    Ok(mut res) => {
                        res.name = "..".to_string();
                        self.items.push(res);
                        self.total += target.blocks;
                    }
                    _ => {}
                };
            }

            for file in files {
                let file_name = file.file_name().unwrap_or_default();
                match Entity::new(file.clone()) {
                    Ok(entity) => {
                        // if flags
                        self.total += if (entity.is_hidden && flags.all) || !entity.is_hidden {
                            entity.blocks
                        } else {
                            0
                        };
                        self.items.push(entity);
                    }
                    Err(err) => {
                        handle_ls_erros(err, file_name.display().to_string());
                    }
                };
            }
        }

        self.items.sort_by(|a, b| {
            let file_a = a.name.strip_prefix(".").unwrap_or(&a.name);
            let file_b = b.name.strip_prefix(".").unwrap_or(&b.name);
            file_a.cmp(&file_b)
        });

        if flags.all {
            // let current =
        }

        // let current
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
    minor: Option<u32>,
    major: Option<u32>,
    size: String,
    time: String,
    name: String,
    blocks: u64,
    is_hidden: bool,
    link_target: Option<PathBuf>,
    is_classified: bool,
    is_long: bool,
}

impl Entity {
    fn new(path: PathBuf) -> Result<Self, Error> {
        let metadata = fs::symlink_metadata(path.clone())?;

        let mut new = Self {
            parent: get_parent(path.clone()),
            path: path.clone(),
            name: path
                .file_name()
                .unwrap_or(Default::default())
                .to_string_lossy()
                .to_string(),
            blocks: metadata.blocks() / 2,
            ..Default::default()
        };
        new.is_hidden = new.name.starts_with(".");
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

fn read_dir(path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let dir = match fs::read_dir(path) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    // println!("try to read the file );

    let mut entries = Vec::new();

    for elem in dir {
        match elem {
            Ok(dir_entry) => {
                entries.push(dir_entry.path());
            }
            Err(err) => println!("- entry err: {:?}", err),
        };
    }

    Ok(entries)
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

// fn path_exists(arg: &String) -> bool {
//     // let meta = fs::symlink_metadata(path)
//     let path = Path::new(arg);
//     path.exists()
// }

fn get_parent(path: PathBuf) -> PathBuf {
    match path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => {
            return path;
        }
    }
}

fn handle_ls_erros(err: Error, entry: String) {
    match err.kind() {
        ErrorKind::NotFound => {
            eprintln!("ls: cannot access '{:?}': No such file or directory", entry)
        }
        ErrorKind::NotADirectory => {
            eprintln!("ls: cannot access '{:?}': Not a directory", entry)
        }
        _ => eprintln!(
            "ls: cannot access '{:?}': Unexpected error =>> {:?}",
            entry, err
        ),
    };
}
