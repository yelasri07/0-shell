# 0-Shell

A simple, lightweight Unix-like shell written in Rust. This project implements basic shell functionality with support for common Unix commands.

## ✨ Features

- **Interactive Command Line Interface** with colored prompts
- **Built-in Commands** - No need for external programs
- **Quote Handling** - Supports single (`'`) and double (`"`) quotes
- **Tilde Expansion** - `~` expands to your home directory
- **Backslash Escaping** - Escape special characters with `\`
- **Multi-line Input** - Continue commands across multiple lines

## 📦 Commands Supported

| Command | Description | Examples |
|---------|-------------|----------|
| `cat` | Display file contents or read from stdin | `cat file.txt` |
| `cd` | Change directory | `cd /home`, `cd ..`, `cd -` |
| `clear` | Clear the terminal screen | `clear` |
| `cp` | Copy files and directories | `cp file.txt backup.txt` |
| `echo` | Print text to stdout | `echo Hello World` |
| `exit` | Exit the shell | `exit` |
| `ls` | List directory contents | `ls -la`, `ls -F` |
| `mkdir` | Create directories | `mkdir newfolder` |
| `mv` | Move or rename files | `mv old.txt new.txt` |
| `pwd` | Print working directory | `pwd` |
| `rm` | Remove files and directories | `rm file.txt`, `rm -r folder/` |
| `touch` | Create empty file or update timestamps | `touch newfile.txt` |

## 🚀 Getting Started

### Prerequisites

- Rust toolchain (1.70 or newer recommended)
- Unix-like operating system (Linux, macOS, BSD)

### Installation

1. **Clone the repository:**
```bash
git clone <your-repo-url>
cd 0-shell
```

2. **Build the project:**
```bash
cargo build --release
```

3. **Run the shell:**
```bash
cargo run
```

Or run the compiled binary directly:
```bash
./target/release/0-shell
```

## 📖 Usage Examples

### Basic Commands
```bash
# Create a new file
$ touch myfile.txt

# Create a directory
$ mkdir projects

# List files with details
$ ls -la

# Copy a file
$ cp myfile.txt backup.txt

# Move/rename a file
$ mv backup.txt archive.txt

# Remove a file
$ rm archive.txt
```

### Working with Directories
```bash
# Go to home directory
$ cd ~

# Go to a specific path
$ cd /usr/local/bin

# Go back to previous directory
$ cd -

# Go up one level
$ cd ..

# Show current directory
$ pwd
```

### Using Quotes and Escaping
```bash
# Single quotes preserve literal strings
$ echo 'Hello $USER'
# Output: Hello $USER

# Double quotes allow variables (if implemented)
$ echo "Hello World"
# Output: Hello World

# Escape special characters
$ echo "Line 1\nLine 2"

# Multi-word arguments
$ mkdir "My Documents"
```

### Advanced ls Options
```bash
# Show all files including hidden
$ ls -a

# Long format with details
$ ls -l

# Classify files with symbols
$ ls -F

# Combine options
$ ls -laF
```

### File Operations
```bash
# Display file content
$ cat readme.txt

# Copy entire directory (recursive)
$ cp -r source_dir/ dest_dir/

# Remove directory and contents
$ rm -r old_folder/

# Create multiple files at once
$ touch file1.txt file2.txt file3.txt
```

## 🏗️ Project Structure

```
rust-shell/
├── src/
│   ├── main.rs           # Entry point, main loop
│   ├── shell.rs          # Shell struct and input parsing
│   ├── utils.rs          # Utility functions
│   └── commands/         # Command implementations
│       ├── mod.rs        # Command module exports
│       ├── cat.rs        # cat command
│       ├── cd.rs         # cd command
│       ├── clear.rs      # clear command
│       ├── cp.rs         # cp command
│       ├── echo.rs       # echo command
│       ├── exit.rs       # exit command
│       ├── ls.rs         # ls command with -l, -a, -F flags
│       ├── mkdir.rs      # mkdir command
│       ├── mv.rs         # mv command
│       ├── pwd.rs        # pwd command
│       ├── rm.rs         # rm command with -r flag
│       └── touch.rs      # touch command
└── Cargo.toml            # Project dependencies
```

## 🔧 Dependencies

- **colored** - Terminal color output
- **chrono** - Date and time handling
- **chrono-tz** - Timezone support
- **users** - User/group information
- **xattr** - Extended file attributes
- **filetime** - File timestamp manipulation
- **libc** - Low-level system calls

## 🎯 Features in Detail

### Quote Handling
- **Single quotes (`'`)**: Preserve literal strings
- **Double quotes (`"`)**: Allow escape sequences
- **Multiline strings**: Automatically prompt for continuation

### Path Expansion
- `~` expands to `$HOME`
- `~/Documents` expands to `$HOME/Documents`
- `cd -` returns to previous directory

### ls Command Features
- `-l`: Long format (permissions, owner, size, date)
- `-a`: Show hidden files (starting with `.`)
- `-F`: Classify files with symbols (`/` for dirs, `*` for executables, etc.)

### Error Handling
- User-friendly error messages
- Proper handling of missing files and permissions
- Prevents dangerous operations (e.g., `rm .` or `rm ..`)

## 🤝 Contributing

Contributions are welcome! Here are some ideas:

- Add more commands (grep, find, etc.)
- Implement pipes (`|`) and redirects (`>`, `<`)
- Add command history (up/down arrows)
- Support for environment variables
- Tab completion
- Background processes (`&`)

## 📝 License

This project is open source and available under the MIT License.

## 🐛 Known Limitations

- No pipe (`|`) or redirection (`>`, `<`) support yet
- No command history
- No tab completion
- No background job control
- Limited to Unix-like systems

## 💡 Tips

- Use `Ctrl+D` (EOF) to exit the shell
- Quotes are necessary for filenames with spaces
- The `-r` flag is required to remove directories with `rm`
- Use `cd -` to quickly switch between two directories

---

**Happy Shelling! 🚀**