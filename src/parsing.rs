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
        if !arg.is_empty() {
            self.args.push(arg);
        }
    }

    pub fn get_cmd(&self) -> String {
        self.cmd.clone()
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn set_cmd(&mut self, value: String) {
        self.cmd = value
    }

    pub fn set_args(&mut self, value: Vec<String>) {
        self.args = value;
    }

    pub fn parse_cmd(&mut self, input: &str) -> Result<(), String> {
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

    pub fn parse_args(&mut self, input: &str) -> Result<(), String> {
        let mut is_quotes = false;
        let mut arg = String::new();
        let mut quote: char = '"';

        for ch in input.chars() {
            if ch == quote && is_quotes {
                is_quotes = false;
                self.add_arg(arg.clone());
                arg.clear();
                continue;
            }

            if (ch == '"' || ch == '\'') && !is_quotes {
                quote = ch;
                is_quotes = true;
                continue;
            }

            if ch == ' ' && !is_quotes && !arg.is_empty() {
                self.add_arg(arg.clone().trim().to_string());
                arg.clear();
                continue;
            }

            arg.push(ch);
        }

        if !arg.is_empty() {
            self.args.push(arg.clone().trim().to_string());
        }

        Ok(())
    }
}
