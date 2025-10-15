use std::fs;
use std::io;
use std::collections::{HashSet, HashMap};
use std::path::{Path, PathBuf};

pub fn ls_handler(args: Vec<String>, current_path: String) {
    let valid_flags: Vec<char> = vec!['l', 'a', 'F'];

    let mut flags = HashSet::new();
    let mut entities = Vec::new();

    for arg in args {
        if !arg.starts_with('-') {
            if arg == "." {
                entities.push(current_path.clone());
            } else {
                entities.push(arg);
            }
            continue
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
    list_items(flags.into_iter().collect(),entities , current_path);

}

#[derive(Debug,Default, Eq, PartialEq, Clone)]
enum EntityType {
    #[default] File,
    Dir,
    Executable, 
    SymLink,
}


#[derive(Debug,Default, Eq, PartialEq, Clone)]
struct Entity {
    path: PathBuf,
    file_type: EntityType,
    permissions:String,
    uid: String,
    gid: String,
    size: String,
    time: String,
    name: String,
}

impl Entity {
    fn new(path: PathBuf) -> Self {
        Self {
            path:path.clone(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            ..Default::default()
        }
    }

    fn classify(&mut self, _is_long_listing: bool)  {
        todo!();
    }

    fn metadata(&mut self) {
        let metadata = fs::symlink_metadata(self.path.clone()).expect("REASON");
        println!("{metadata:?}");
    }
}

fn list_items(flags:Vec<char>, entities: Vec<String>, current_path: String) {
    let have_flags = flags.is_empty();
    let have_entities = entities.is_empty();

    let mut entities_to_display: HashMap<String, Vec<Entity>> = HashMap::new();

    for (index, entity) in entities.iter().enumerate()  {
        let full_path = if !entity.starts_with("/") {
            format!("{}/{}", current_path,entity)
        } else {
            entity.to_string()
        };

        if !path_exists(full_path.clone()) {
            eprintln!("ls: cannot access 'target': No such file or directory");
            continue;
        }

        if is_dir(full_path.clone()) {
            let files = fs::read_dir(&full_path).unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>().unwrap();

            let entities: Vec<Entity> = files.into_iter()
            .map(|p| Entity::new(p))
            .collect();
            entities_to_display.insert(entity.to_string(), entities);
        } else {
            let file = Path::new(&full_path).to_path_buf();
            entities_to_display.insert(entity.to_string(), vec![Entity::new(file)]);
        }

        for (key, val) in &entities_to_display {
            if entities.len() > 1 || is_dir(format!("{}/{}", current_path, key.clone())) {
                println!("{}:", key);
            }

            let names: Vec<String> = val.iter()
                .map(|entity| entity.name.clone())
                .collect();

            println!("{}", names.join(" "));

            println!();
        }

    }
}

// utitlies : 

fn path_exists(arg : String) -> bool {
    let path = Path::new(&arg);
    path.exists()
}

fn is_dir(path: String) -> bool {
    let path = Path::new(&path);
    path.is_dir()
}

fn format_permissions(_permissions: String) -> String  {
    todo!();
}