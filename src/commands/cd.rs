use std::env;

pub fn cd_handler(args: Vec<String>, prev_path: &String) -> String {
    let mut new_dir: &str = &args.join(" ");

    if new_dir.is_empty() || new_dir == "~" {
        new_dir = "/home/";
    }

    let path = match env::current_dir() {
        Ok(p) => format!("{:?}", p),
        Err(e) => {
            println!("{}", e);
            "".to_string()
        }
    };

    if new_dir == "-" {
        if prev_path.is_empty() {
            println!("cd: OLDPWD not set");
            return "".to_string();
        }
        println!("{}", prev_path);
        new_dir = prev_path;
    }

    if let Err(e) = env::set_current_dir(new_dir) {
        println!("{}", e);
        return "".to_string();
    }

    path
}
