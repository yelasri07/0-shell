use std::path::PathBuf;

use crate::{commands::*, utils::read_line};

#[derive(Default)]
pub struct Shell {
    arg: String,
    pub args: Vec<String>,
    pub home: String,
    is_quotes: bool,
    quotes_type: char,
    is_backslash: bool,
    prev_path: PathBuf,
    pub current_path: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_arg_with_quotes(&mut self, arg: String) {
        if !arg.is_empty() {
            self.args.push(arg);
            self.arg.clear();
        }
    }

    pub fn add_arg(&mut self) {
        if !self.arg.is_empty() {
            let mut arg = self.arg.clone().trim().to_string();
            if arg == "~" || (arg.starts_with("~") && arg.chars().nth(1).unwrap_or(' ') == '/') {
                arg = if arg.len() > 1 {
                    self.home.clone() + &arg[1..arg.len()]
                } else {
                    self.home.clone()
                }
            }
            self.args.push(arg);
            self.arg.clear();
        }
    }

    pub fn set_args(&mut self, value: Vec<String>) {
        self.args = value;
    }

    pub fn set_arg(&mut self, value: String) {
        self.arg = value
    }

    pub fn set_current_path(&mut self, value: PathBuf) {
        if !value.as_os_str().is_empty() {
            self.current_path = value
        }
    }

    pub fn set_quotes_type(&mut self, value: char) {
        self.quotes_type = value
    }

    pub fn set_home(&mut self, value: String) {
        self.home = value
    }

    pub fn parse_input(&mut self, input: &str) -> Result<(), String> {
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
                self.quotes_type = '"';
                if input.chars().nth(i + 1).unwrap_or(' ') == ' ' {
                    self.add_arg_with_quotes(self.arg.clone());
                }
                continue;
            }

            if ch == ' ' && !self.is_quotes && !self.arg.is_empty() {
                self.add_arg();
                continue;
            }

            if ch != ' ' || (ch == ' ' && self.is_quotes) {
                self.arg.push(ch);
            }
        }

        if self.is_backslash {
            self.is_backslash = false;
            let (input, _) = read_line(">", &self.home);
            let _ = self.parse_input(input.as_str());
        }

        if !self.arg.is_empty() && !self.is_quotes {
            self.add_arg();
        }

        if self.is_quotes {
            let quote_text = if self.quotes_type == '\'' {
                "quote>"
            } else {
                "dquote>"
            };
            let (input, n_bytes) = read_line(quote_text, &self.home);
            if n_bytes == 0 {
                self.is_quotes = false;
                return Err(format!(
                    "\nunexpected EOF while looking for matching `{}'\nsyntax error: unexpected end of file",
                    self.quotes_type
                ));
            }
            self.arg.push('\n');
            let _ = self.parse_input(input.as_str());
        }

        Ok(())
    }

    pub fn run(&mut self) {
        let empty_cmd = &"".to_string();
        let cmd: &str = &self.args.iter().next().unwrap_or(empty_cmd);

        if cmd.is_empty() {
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
                let (prev_path, current_path) = cd_handler(
                    args,
                    self.prev_path.to_path_buf(),
                    self.current_path.to_path_buf(),
                    self.home.clone(),
                );
                self.prev_path = prev_path;
                self.set_current_path(current_path);
            }
            "cp" => cp_handler(args),
            "echo" => echo_handler(args),
            "exit" => exit_handler(),
            "ls" => ls_handler(args, self.current_path.clone()),
            "mkdir" => mkdir_handler(args, self.current_path.clone()),
            "mv" => mv_handler(args),
            "pwd" => pwd_handler(&self.current_path),
            "clear" => clear_handler(),
            "rm" => rm_handler(args),
            _ => eprintln!("Command {} not found", cmd),
        }
    }
}
