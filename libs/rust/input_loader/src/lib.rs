use std::io::{self, BufRead};

#[must_use]
/// # Panics
/// if input is not found
pub fn load_input() -> Vec<String> {
    // Read input
    let input = io::stdin();
    // Create input buffer
    let mut input_buffer = Vec::new();
    // Read int buffer
    for line in input.lock().lines() {
        input_buffer.push(line.unwrap());
    }

    input_buffer
}
