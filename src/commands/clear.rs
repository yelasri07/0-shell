use std::io::{stdout, Write};

pub fn clear_handler() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}