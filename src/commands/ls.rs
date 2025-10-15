use std::fs;
use std::io;
use std::collections::HashSet;

// ls a file
// ls a dir
// ls mutiple 


pub fn ls_handler(args: Vec<String>, current_path: String) {
    let valid_flags: Vec<char> = vec!['l', 'a', 'F'];

    let mut flags = HashSet::new();
    let mut entities = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                if !valid_flags.contains(&ch) {
                    eprintln!("ls: invalid option -- '{}'", ch);
                    break;
                }
                flags.insert(ch);
            }
        } else {
            entities.push(arg);
        }
    }

    if entities.is_empty() {
        entities.push(current_path.clone());
    }


    println!("Current Path: {}", current_path);
    println!("Flags: {:?}", flags);
    println!("Files and folders: {:?}", entities);

    // get_meta_data(".".to_string());
}

fn is_valid_flags(flags: String) -> bool {
    let _valid_flags = ["a","l","F"];
    for flag in flags.split("") {
        println!("flag : {:?}", flag);
    }
    true
}



// fn list_dir(dir: String) -> Vec<String> {

// }

// {file_type:?}{permissions:?} {owner of the file:?} {group owner:?} {size:?} {:?modification time} {:name of the file}
fn get_meta_data(entity: String) -> Vec<String> {
    let meta_data = Vec::new();
    let sym_metadata = fs::symlink_metadata(entity).expect("REASON");
    println!("{sym_metadata:?}");
    meta_data
}


// print output.

// -a : include hidding files in the listing
// -l : get detail information about files and directories

// 