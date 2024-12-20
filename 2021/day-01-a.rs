use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    let mut prev = input
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut res = 0;
    for line in input.lock().lines() {
        let current = line.unwrap().parse::<usize>().unwrap();
        if current > prev {
            res += 1;
        }
        prev = current;
    }
    println!("{res}");
}
