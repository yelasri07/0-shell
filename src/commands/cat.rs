use std::fs;
use std::io::Read;

pub fn cat_handler(args: Vec<String>) {
    // println!("DEBUG => args = {:?}", args);

    if args.is_empty() {
        eprintln!("Usage: cat <filename>");
        return;
    }

    for filename in &args {
        //println!("DEBUG: trying to open {}", filename);

        match fs::File::open(filename) {
            Ok(mut file) => {
                //println!("DEBUG: successfully opened {}", filename);

                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    eprintln!("Failed to read file '{}': {}", filename, e);
                } else {
                    //println!("DEBUG: read ok, printing content");
                    print!("{}", contents);
                    if !contents.ends_with('\n') {
                        println!();
                    }
                }
            }

            Err(e) => eprintln!("Failed to open file '{}': {}", filename, e),
        }
    }
}
