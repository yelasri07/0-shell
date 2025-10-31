use chrono::{DateTime, Duration, Utc};
use chrono_tz::Africa::Casablanca;
use core::fmt;
use std::fs::{self};
use std::fs::{Metadata, metadata};
use std::io::{Error};
use std::os::unix::fs::*;
use std::path::PathBuf;
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
    targets_len: usize,
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
            targets_len: 0,
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
        ls.targets_len = target_args.len();

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
            let current = PathBuf::from(format!("{}/.", self.current_path.display().to_string()));
            let current_dir = match Entity::new(current) {
                Ok(entity) => entity,
                Err(err) => {
                    handle_ls_erros(err, ".".to_string());
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
        // let targets_len = self.targets.len();
        for (index, mut target) in self.targets.clone().into_iter().enumerate() {
            let mut list = List::new(target.0.clone());
            if index > 0 && target.1.file_type == EntityType::Dir {
                println!("")
            }

            if self.targets_len > 1 && target.1.file_type == EntityType::Dir {
                println!("{}:", list.header);
            }

            list.get_items(&mut target.1, self.flags.clone());
            if self.flags.long && target.1.file_type == EntityType::Dir {
                println!("total {}:", list.total);
            }

            for (_, file) in list.items.iter_mut().enumerate() {
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
        let end_with_slash = target.path.display().to_string().ends_with("/");
        let is_symlink = target.file_type == EntityType::SymLink;
        let mut is_dir = false;

        let files = match read_dir(target.path.clone(), flags.all) {
            Ok(res) => {
                if is_symlink && (flags.classify || flags.long) {
                    if end_with_slash {
                        is_dir = true
                    }
                } else {
                    is_dir = true;
                }
                res
            }
            Err(err) => {
                handle_ls_erros(err, target.name.clone());
                return;
            }
        };

        if !is_dir {
            self.items.push(target.clone());
            return;
        }

        for file in files {
            let file_name = file.file_name().unwrap_or_default();
            match Entity::new(file.clone()) {
                Ok(mut entity) => {
                    if file == target.path {
                        entity.name = ".".to_string();
                    }

                    if file == target.parent {
                        entity.name = "..".to_string();
                    }

                    self.total += entity.blocks;
                    self.items.push(entity);
                }

                Err(err) => {
                    handle_ls_erros(err, file_name.display().to_string());
                }
            };
        }

        self.items.sort_by(|a, b| {
            let file_a = a
                .name
                .strip_prefix(".")
                .unwrap_or(&a.name)
                .to_ascii_lowercase();
            let file_b = b
                .name
                .strip_prefix(".")
                .unwrap_or(&b.name)
                .to_ascii_lowercase();
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
                .display()
                .to_string(),
            blocks: metadata.blocks() / 2,
            ..Default::default()
        };
        new.file_type = get_file_type(metadata.permissions().mode());
        new.link_target = read_link(new.clone());
        Ok(new)
    }

    fn long_list(&mut self) {
        self.is_long = true;
        let metadata = fs::symlink_metadata(self.path.clone()).expect("");

        let mode = metadata.permissions().mode();
        let permissions = get_permissions(mode, self.path.clone());
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
        self.time = get_modified_time(metadata);

        if let Some(data) = major_minor(self.clone()) {
            self.major = Some(data.0);
            self.minor = Some(data.1);
        } else {
            self.major = None;
            self.minor = None;
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut name = self.name.to_owned();
        let size = match self.file_type {
            EntityType::BlockDevice | EntityType::CharacterDevice => {
                format!(
                    "{}, {}",
                    self.major.unwrap_or_default(),
                    self.minor.unwrap_or_default()
                )
            }
            _ => self.size.clone(),
        };
        let (symbol, mut sufix) = get_file_type_symbols(self.file_type.clone());

        if self.file_type == EntityType::SymLink {
            if let Some(path) = self.link_target.clone() {
                match metadata(path) {
                    Ok(metada) => {
                        sufix = if self.is_long {
                            get_file_type_symbols(get_file_type(metada.mode())).1
                        } else {
                            sufix
                        };
                    }
                    Err(_) => sufix = "",
                };
            }
        }

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
                mode, self.nlink, self.uid, self.gid, size, self.time, name, sufix
            )
        } else {
            writeln!(f, "{}{}", self.name, sufix)
        }
    }
}

// utitlies :
fn get_modified_time(metadata: Metadata) -> String {
    let dt = DateTime::<Utc>::from_timestamp(metadata.clone().mtime(), 0);
    let datetime = dt.unwrap().with_timezone(&Casablanca);
    let current_date_time = Utc::now().with_timezone(&Casablanca);

    let six_months_ago = current_date_time - Duration::days(183);

    if datetime > six_months_ago && datetime < current_date_time {
        return datetime.format("%b %e %H:%M").to_string();
    }

    datetime.format("%b %e  %Y").to_string()
}

fn get_file_type_symbols(file_type: EntityType) -> (&'static str, &'static str) {
    match file_type {
        EntityType::File => ("-", ""),
        EntityType::Dir => ("d", "/"),
        EntityType::SymLink => ("l", "@"),
        EntityType::CharacterDevice => ("c", ""),
        EntityType::BlockDevice => ("b", ""),
        EntityType::Fifo => ("p", "|"),
        EntityType::Socket => ("s", "="),
        EntityType::Executable => ("-", "*"),
        EntityType::None => ("", ""),
    }
}

fn get_permissions(mode: u32, path: PathBuf) -> String {
    let mut permissions = String::new();

    let owner = (mode >> 6) & 0o7;
    let group = (mode >> 3) & 0o7;
    let other = mode & 0o7;

    let setuid = mode & 0o4000 != 0;
    let setgid = mode & 0o2000 != 0;
    let sticky_bit = mode & 0o1000 != 0;

    permissions.push(if owner & 0o4 != 0 { 'r' } else { '-' });
    permissions.push(if owner & 0o2 != 0 { 'w' } else { '-' });
    permissions.push(if setuid {
        if owner & 0o1 != 0 { 's' } else { 'S' }
    } else {
        if owner & 0o1 != 0 { 'x' } else { '-' }
    });

    permissions.push(if group & 0o4 != 0 { 'r' } else { '-' });
    permissions.push(if group & 0o2 != 0 { 'w' } else { '-' });
    permissions.push(if setgid {
        if group & 0o1 != 0 { 's' } else { 'S' }
    } else {
        if group & 0o1 != 0 { 'x' } else { '-' }
    });

    permissions.push(if other & 0o4 != 0 { 'r' } else { '-' });
    permissions.push(if other & 0o2 != 0 { 'w' } else { '-' });
    permissions.push(if sticky_bit {
        if other & 0o1 != 0 { 't' } else { 'T' }
    } else {
        if other & 0o1 != 0 { 'x' } else { '-' }
    });

    let has_acl = xattr::list(path)
        .ok()
        .map(|attrs| {
            attrs
                .into_iter()
                .any(|name| name.to_str() == Some("system.posix_acl_access"))
        })
        .unwrap_or(false);
    if has_acl {
        permissions.push('+');
    }

    permissions
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

fn read_dir(path: PathBuf, all: bool) -> Result<Vec<PathBuf>, Error> {
    let dir = match fs::read_dir(&path) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };

    let mut entries: Vec<PathBuf> = Vec::new();

    if all {
        entries.push(path.clone());
        entries.push(get_parent(path.clone()));
    }

    for elem in dir {
        match elem {
            Ok(dir_entry) => {
                if !all && dir_entry.file_name().display().to_string().starts_with(".") {
                    continue;
                }
                entries.push(dir_entry.path());
            }
            Err(_) => {}
        };
    }

    Ok(entries)
}

fn major_minor(entity: Entity) -> Option<(u32, u32)> {
    if entity.file_type != EntityType::CharacterDevice
        && entity.file_type != EntityType::BlockDevice
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
    let mut file_type = match mode & 0o170000 {
        0o100000 => EntityType::File,
        0o040000 => EntityType::Dir,
        0o120000 => EntityType::SymLink,
        0o020000 => EntityType::CharacterDevice,
        0o060000 => EntityType::BlockDevice,
        0o010000 => EntityType::Fifo,
        0o140000 => EntityType::Socket,
        _ => EntityType::None,
    };

    let is_executable = mode & 0o111 != 0;
    if file_type == EntityType::File && is_executable {
        file_type = EntityType::Executable
    };
    file_type
}

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
        _ => {
            if let Some(raw_os_error) = err.raw_os_error() {
                eprintln!(
                    "ls: cannot access '{}': {}",
                    entry,
                    err.to_string()
                        .replace(&format!(" (os error {})", raw_os_error), "")
                );
            };
        }
    }
}
