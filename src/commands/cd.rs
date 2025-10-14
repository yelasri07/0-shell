use std::env;

pub fn cd_handler(args: Vec<String>, prev_path: &String) -> String {
    let mut new_dir: &str = &args.join(" ");

    if new_dir.is_empty() || new_dir == "~" {
        new_dir = "/home/";
    }

    if new_dir == "-" {
        new_dir = prev_path;
    }

    let path = match env::current_dir() {
        Ok(p) => format!("{:?}", p),
        Err(e) => {
            println!("{}", e);
            "".to_string()
        }
    };
    
    if let Err(e) = env::set_current_dir(new_dir) {
        println!("{}", e);
    }

    path
}
