use crate::{commands::*, utils::read_line};

pub struct Shell {
    arg: String,
    pub args: Vec<String>,
    is_quotes: bool,
    quotes_type: char,
    prev_path: String,
    is_backslash: bool,
    pub current_path: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            arg: String::new(),
            prev_path: String::new(),
            current_path: String::new(),
            args: vec![],
            quotes_type: '"',
            is_quotes: false,
            is_backslash: false,
        }
    }

    pub fn add_arg(&mut self, arg: String) {
        if !arg.is_empty() {
            self.args.push(arg);
        }
    }

    pub fn set_args(&mut self, value: Vec<String>) {
        self.args = value;
    }

    pub fn set_arg(&mut self, value: String) {
        self.arg = value
    }

    pub fn set_current_path(&mut self, value: String) {
        self.current_path = value
    }

    pub fn parse_input(&mut self, input: &str) {
        for (i, ch) in input.chars().enumerate() {
            if self.is_backslash {
                if (ch != '$' && ch != '`' && ch != '"' && ch != '\\') && self.is_quotes {
                    self.arg.push('\\');
                }
                self.arg.push(ch);
                self.is_backslash = false;
                continue;
            }

            if ch == '\\' && self.quotes_type != '\'' {
                self.is_backslash = true;
                continue;
            }

            if (ch == '"' || ch == '\'') && !self.is_quotes {
                self.quotes_type = ch;
                self.is_quotes = true;
                continue;
            }

            if ch == self.quotes_type {
                self.is_quotes = false;
                if input.chars().nth(i + 1).unwrap_or(' ') == ' ' {
                    self.add_arg(self.arg.clone());
                    self.arg.clear();
                }
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

        if self.is_backslash {
            self.is_backslash = false;
            let (input, _) = read_line(">");
            self.parse_input(input.as_str());
        }

        if !self.arg.is_empty() && !self.is_quotes {
            self.add_arg(self.arg.clone().trim().to_string());
            self.arg.clear();
        }

        if self.is_quotes {
            let quote_text = if self.quotes_type == '\'' {
                "quote>"
            } else {
                "dquote>"
            };
            let (input, n_bytes) = read_line(quote_text);
            if n_bytes == 0 {
                self.is_quotes = false;
                eprintln!(
                    "\nunexpected EOF while looking for matching `{}'\nsyntax error: unexpected end of file",
                    self.quotes_type
                );
                return;
            }
            self.arg.push('\n');
            self.parse_input(input.as_str());
        }
    }

    pub fn run(&mut self) {
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

        let empty_cmd = &"".to_string();
        let cmd: &str = &self.args.iter().next().unwrap_or(empty_cmd);

        if cmd.is_empty() {
            return;
        }

        if !cmds.contains(&cmd.to_string()) {
            println!("Command {} not found", cmd);
            return;
        }

        let args: Vec<String> = if self.args.len() > 1 {
            self.args[1..self.args.len()].to_vec()
        } else {
            vec![]
        };

        match cmd {
            "cat" => cat_handler(args),
            "cd" => {
                self.prev_path = cd_handler(args, &self.prev_path)
                    .trim_matches('"')
                    .to_string()
            }
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
