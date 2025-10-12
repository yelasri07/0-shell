use crate::{commands::*, read_line::read_line};

pub struct Shell {
    cmd: String,
    arg: String,
    args: Vec<String>,
    is_quotes: bool,
    quotes_type: char,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            cmd: String::new(),
            args: vec![],
            arg: String::new(),
            is_quotes: false,
            quotes_type: '"',
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

    pub fn set_cmd(&mut self, value: String) {
        self.cmd = value
    }

    pub fn set_args(&mut self, value: Vec<String>) {
        self.args = value;
    }

    pub fn set_arg(&mut self, value: String) {
        self.arg = value
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

    pub fn parse_args(&mut self, input: &str) {
        for ch in input.chars() {
            if ch == self.quotes_type && self.is_quotes {
                self.is_quotes = false;
                self.add_arg(self.arg.clone());
                self.arg.clear();
                continue;
            }

            if (ch == '"' || ch == '\'') && !self.is_quotes {
                self.quotes_type = ch;
                self.is_quotes = true;
                continue;
            }

            if ch == ' ' && !self.is_quotes && !self.arg.is_empty() {
                self.add_arg(self.arg.clone().trim().to_string());
                self.arg.clear();
                continue;
            }

            if ch != ' ' || (ch == ' ' && self.is_quotes) {
                self.arg.push(ch);
            }
        }

        if !self.arg.is_empty() && !self.is_quotes {
            self.add_arg(self.arg.clone().trim().to_string());
        }

        if self.is_quotes {
            let input = read_line("> ");
            self.arg.push('\n');
            self.parse_args(&input);
        }
    }

    pub fn run(&self) {
        let cmd: &str = &self.cmd;
        let args = self.args.clone();

        match cmd {
            "cat" => cat_handler(args),
            "cd" => cd_handler(args),
            "cp" => cp_handler(args),
            "echo" => echo_handler(args),
            "exit" => exit_handler(args),
            "ls" => ls_handler(args),
            "mkdir" => mkdir_handler(args),
            "mv" => mv_handler(args),
            "pwd" => pwd_handler(args),
            _ => rm_handler(args),
        }
    }
}
