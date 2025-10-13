use std::fs;
use std::io;

// ls a file
// ls a dir
// ls mutiple 

#[derive(Debug,Default)]

pub fn handle_ls(args: Vec<String>) {
    let flags = args.iter().filter(|arg| arg.starts_with("-")) ;
    let entities =  args.iter().filter(|arg| !arg.starts_with("-"))  ;
    // println

    get_meta_data(".".to_string());
}

fn is_valid_flags(flags: String) -> bool {
    let valid_flags = ["a","l","F"];
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