use std::fs;
use std::io::{self, BufRead, Read, Write};

pub fn cat_handler(args: Vec<String>) {
    if args.is_empty() {
        // eprintln!("Usage: cat <filename>");
        read_from_stdin();
        return;
    }

    for filename in &args {
        //println!("DEBUG: trying to open {}", filename);
        //cat -
        if filename == "-" {
            //Ensure that the output from previous files is printed before reading from stdin
             io::stdout().flush().unwrap();
             read_from_stdin();
        }else{
            match fs::File::open(filename) {
                Ok(mut file) => {
                    //println!("DEBUG: successfully opened {}", filename);
                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        eprintln!("Failed to read file '{}': {}", filename, e);
                    }else {
                        //println!("DEBUG: read ok, printing content");
                        print!("{}", contents);
                    }
                }
                Err(e) => eprintln!("Failed to open file '{}': {}", filename, e),
            }
        }
    }
    println!();
}

fn read_from_stdin() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        match line {
            Ok(content) => {
                writeln!(stdout, "{}", content).unwrap();
            }
            Err(e) => {
                eprintln!("cat: error reading from stdin: {}", e);
                break;
            }
        }
    }
}