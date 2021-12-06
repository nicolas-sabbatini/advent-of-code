use std::io::{self, BufRead};

fn main() {
    // Read input
    let input = io::stdin();

    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    // Do comparations
    for line in input.lock().lines() {
        let line = line.unwrap();

        let action = &line[0..line.len() - 2];
        let value = line[line.len() - 1..line.len()].parse::<usize>().unwrap();

        match action {
            "up" => {
                aim -= value;
            }
            "down" => {
                aim += value;
            }
            "forward" => {
                horizontal += value;
                depth += value * aim;
            }
            _ => {}
        }
    }

    println!("{}", horizontal * depth);
}
