use std::io::{self, BufRead};

fn main() {
    // Read input
    let input = io::stdin();
    // Create a vector to store mesurment
    let mut mesurment_vec = Vec::new();
    // Init start
    let mut res = 0;
    // Do comparations
    for line in input.lock().lines() {
        let mes = line.unwrap().parse::<usize>().unwrap();
        mesurment_vec.push(mes);
    }
    // Calculate the mesurments
    for i in 0..mesurment_vec.len() - 3 {
        let mut current = 0;
        let mut next = 0;

        for k in 0..3 {
            current += mesurment_vec[i + k];
            next += mesurment_vec[i + 1 + k];
        }

        if current < next {
            res += 1;
        }
    }
    // Print result
    println!("{}", res);
}
