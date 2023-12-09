#![allow(clippy::cast_possible_wrap)]
use input_loader::load_input;

fn predict_next_number(numbers: &Vec<isize>) -> isize {
    let mut secuences = vec![numbers.clone()];
    while secuences.last().unwrap().iter().any(|x| *x != 0) {
        let mut new_sequence = Vec::new();
        for sequence in secuences.last().unwrap().windows(2) {
            new_sequence.push(sequence[1] - sequence[0]);
        }
        secuences.push(new_sequence);
    }
    let mut res = 0;
    for secuence in secuences.iter().rev() {
        let last = secuence.last().unwrap();
        res += last;
    }
    res
}

fn main() {
    // Load input
    let input = load_input();
    let mut readings = Vec::new();
    for line in input {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        readings.push(numbers);
    }
    println!(
        "{}",
        readings.iter().map(predict_next_number).sum::<isize>()
    );
}
