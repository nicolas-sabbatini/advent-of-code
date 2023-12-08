use std::collections::{HashMap, VecDeque};

use input_loader::load_input;

#[derive(Debug, Clone, Copy)]
enum Instructions {
    Left,
    Right,
}

impl Instructions {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid instruction"),
        }
    }

    fn aplay_instruction(self, direction: &Direction) -> String {
        match self {
            Self::Left => direction.left.clone(),
            Self::Right => direction.right.clone(),
        }
    }
}

#[derive(Debug)]
struct Direction {
    left: String,
    right: String,
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut input_iter = input.iter();
    let instructions = input_iter
        .next()
        .unwrap()
        .chars()
        .map(Instructions::from_char)
        .collect::<VecDeque<Instructions>>();
    let mut directions = HashMap::new();
    let mut start_positions = Vec::new();
    for line in input_iter.skip(1) {
        let line = line.split(" = ").collect::<Vec<&str>>();
        let index = line[0];

        if index.ends_with('A') {
            start_positions.push(index.to_owned());
        }

        let cordinates = line[1].replace(['(', ')'], "");
        let cordinates = cordinates.split(", ").collect::<Vec<&str>>();
        directions.insert(
            index.to_owned(),
            Direction {
                left: cordinates[0].to_owned(),
                right: cordinates[1].to_owned(),
            },
        );
    }

    let mut steps: Vec<usize> = vec![0; start_positions.len()];
    for (step, current_position) in start_positions.iter_mut().enumerate() {
        let mut local_instructions = instructions.clone();
        loop {
            let go_to = directions.get(current_position).unwrap();
            *current_position = local_instructions[0].aplay_instruction(go_to);
            steps[step] += 1;
            local_instructions.rotate_left(1);
            if current_position.ends_with('Z') {
                break;
            }
        }
    }

    println!("{}", steps.iter().fold(1, |acc, x| lcm(acc, *x)));
}
