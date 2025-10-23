pub fn clear_handler() {
    // ANSI escape code to clear
    print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[3J");

}