use std::collections::VecDeque;

use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut res = 0;
    let mut queue = VecDeque::from(vec![1; input.len()]);
    for line in input {
        let line = line.split(": ").collect::<Vec<&str>>();
        let line = line[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = line[0]
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let card_numbers = line[1]
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let card_copies = queue.pop_front().unwrap();
        let mut index = 0;
        for winning_number in winning_numbers {
            if card_numbers.contains(&winning_number) {
                queue[index] += card_copies;
                index += 1;
            }
        }
        res += card_copies;
    }
    println!("{res:?}");
}
