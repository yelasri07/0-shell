pub fn clear_handler() {
    // ANSI escape code to clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");

    #[cfg(unix)]
    {
        print!("\x1B[3J\x1B[2J\x1B[1;1H");
    }
    
    #[cfg(windows)]
    {
        print!("\x1B[2J\x1B[1;1H");
    }
}