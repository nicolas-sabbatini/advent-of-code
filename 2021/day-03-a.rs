use std::io::{self, BufRead};

fn main() {
    // Read input
    let input = io::stdin();
    // Create input buffer
    let mut input_buffer = Vec::new();
    // Read int buffer
    for line in input.lock().lines() {
        input_buffer.push(line.unwrap());
    }
    // Get binary size
    let size = input_buffer[0].len();

    // Res variables
    let mut gamma = vec!['0'; size];
    let mut epsilon = vec!['0'; size];

    // calculate variables
    for i in 0..size {
        let mut zero = 0;
        let mut one = 0;
        for input in &input_buffer {
            match input.chars().nth(i).unwrap() {
                '0' => zero += 1,
                '1' => one += 1,
                _ => (),
            }
        }

        if zero < one {
            gamma[i] = '1';
        } else {
            epsilon[i] = '1';
        }
    }

    // Convert from binary to usize and print res
    println!("{}", binary_to_usize(gamma) * binary_to_usize(epsilon));
}

fn binary_to_usize(num: Vec<char>) -> usize {
    usize::from_str_radix(&(num.into_iter().collect::<String>()), 2).unwrap()
}
