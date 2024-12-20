use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    let mut mesurment_vec = Vec::new();
    let mut res = 0;
    for line in input.lock().lines() {
        let mesurment = line.unwrap().parse::<usize>().unwrap();
        mesurment_vec.push(mesurment);
    }
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
    println!("{res}");
}
