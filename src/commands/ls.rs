use chrono::{DateTime, Local};
use core::fmt;
use std::collections::HashSet;
use std::fmt::{Display};
use std::fs;
use std::fs::Metadata;
use std::io;
use std::os::unix::fs::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug, Eq, PartialEq, Default)]
struct Target(String, String);

#[derive(Debug, Default)]
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

        self.


    }

    fn execute() {

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

    println!("ls ==>> {:?}", ls);

    // todo: sort by files then folders.
    // let flags_vec: Vec<char> = flags.clone().into_iter().collect();

    // for (index, entity) in entities.iter().enumerate() {
    //     if is_dir(full_path.clone()) && entities.len() > 1 {
    //         println!("{}:", entity);
    //     }

    //     list_items(flags_vec.clone(), full_path, entity.to_string());

    //     println!("");
    //     if index != entities.len() - 1 {
    //         println!("");
    //     }
    // }
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
    fn new(path: PathBuf) -> Self {
        let mut new = Self {
            path: path.clone(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            ..Default::default()
        };

        let metadata = fs::symlink_metadata(new.path.clone()).expect("REASON");
        new.file_type(metadata);
        new
    }

    fn long_list(&mut self) {
        let metadata = fs::symlink_metadata(self.path.clone()).expect("REASON");
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
        self.get_modified_time(metadata)
    }

    fn get_permissions(&mut self, metadata: Metadata) {
        let permissions_bits = metadata.permissions().mode() & 0o777;
        let permissions: Vec<u32> = format!("{:o}", permissions_bits)
            .split("")
            .filter(|e| !e.is_empty())
            .map(|e| e.parse().unwrap())
            .collect();

        let mut res = Vec::new();
        for permission in permissions {
            let mut nb = permission.clone() as i32;
            if nb - 4 >= 0 {
                nb = nb - 4;
                res.push("r");
            } else {
                res.push("-");
            }
            if nb - 2 >= 0 {
                nb = nb - 2;
                res.push("w");
            } else {
                res.push("-");
            }
            if nb - 1 >= 0 {
                res.push("x");
                self.file_type = if self.file_type == EntityType::File {
                    EntityType::Executable
                } else {
                    self.file_type.clone()
                };
            } else {
                res.push("-");
            }
        }
        self.permissions = res.join("");
    }

    fn file_type(&mut self, metadata: Metadata) {
        let mode = metadata.permissions().mode();
        self.file_type = match mode & 0o170000 {
            0o100000 => EntityType::File,
            0o040000 => EntityType::Dir,
            0o120000 => EntityType::SymLink,
            0o020000 => EntityType::CharacterDevice,
            0o060000 => EntityType::BlockDevice,
            0o010000 => EntityType::Fifo,
            0o140000 => EntityType::Socket,
            _ => todo!(),
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

fn list_items(flags: Vec<char>, full_path: String, entity: String) {
    let mut list: Vec<Entity> = Vec::new();

    if !path_exists(&full_path) {
        eprintln!("ls: cannot access '{}': No such file or directory", entity);
        return;
    }

    if !is_dir(full_path.clone()) {
        let file = Path::new(&full_path).to_path_buf();
        list.push(Entity::new(file));
    } else {
        // todo : handle the errors alhmar
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
            EntityType::Fifo => ("p", ""),
            EntityType::Socket => ("s", ""),
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

fn path_exists(arg: &String) -> bool {
    let path = Path::new(arg);
    path.exists()
}

fn is_dir(path: String) -> bool {
    let path = Path::new(&path);
    path.is_dir()
}
