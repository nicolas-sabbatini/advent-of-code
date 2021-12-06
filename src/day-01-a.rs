use std::io::{self, BufRead};

fn main() {
    // Read input
    let input = io::stdin();
    // Get first read
    let mut prev = input
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    // Init start
    let mut res = 0;
    // Do comparations
    for line in input.lock().lines() {
        // Get current meurment
        let current = line.unwrap().parse::<usize>().unwrap();
        if current > prev {
            res += 1;
        }
        // Change mesurment
        prev = current;
    }
    // Print result
    println!("{}", res);
}
