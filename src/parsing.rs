pub struct Parsing {
    cmd: String,
    args: Vec<String>,
}

impl Parsing {
    pub fn new() -> Self {
        Self {
            cmd: String::new(),
            args: vec![],
        }
    }

    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg.trim().to_string());
    }

    pub fn get_cmd(&self) -> String {
        self.cmd.clone()
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn set_cmd(&mut self, input: &str) -> Result<(), String> {
        let cmds = [
            "cat".to_string(),
            "cd".to_string(),
            "cp".to_string(),
            "echo".to_string(),
            "exit".to_string(),
            "ls".to_string(),
            "mkdir".to_string(),
            "mv".to_string(),
            "pwd".to_string(),
            "rm".to_string(),
        ];

        let mut flag = String::new();
        for ch in input.chars() {
            if ch == ' ' {
                break;
            }

            flag.push(ch);
        }

        if !cmds.contains(&flag) {
            return Err(format!("Command {} not found", flag));
        }

        self.cmd = flag;

        Ok(())
    }

    pub fn set_args(&mut self, input: &str) {
        let mut is_start_with_quotes = false;
        let mut arg = String::new();

        for ch in input.chars() {
            if ch == '"' && is_start_with_quotes {
                is_start_with_quotes = false;
                self.add_arg(arg.clone());
                arg.clear();
                continue;
            }

            if ch == '"' {
                is_start_with_quotes = true;
                continue;
            }

            if ch == ' ' && !is_start_with_quotes && !arg.is_empty() {
                self.add_arg(arg.clone());
                arg.clear();
                continue;
            }

            arg.push(ch);
        }

        if !arg.is_empty() {
            self.args.push(arg.clone().trim().to_string());
        }

        println!("=>> {:?}", self.args);
    }
}
