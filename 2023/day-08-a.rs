use std::collections::{HashMap, VecDeque};

use input_loader::load_input;

#[derive(Debug)]
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

    fn aplay_instruction(&self, direction: &Direction) -> String {
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

fn main() {
    // Load input
    let input = load_input();
    let mut input_iter = input.iter();
    let mut instructions = input_iter
        .next()
        .unwrap()
        .chars()
        .map(Instructions::from_char)
        .collect::<VecDeque<Instructions>>();
    let mut directions = HashMap::new();
    for line in input_iter.skip(1) {
        let line = line.split(" = ").collect::<Vec<&str>>();
        let index = line[0];

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

    let mut steps = 0;
    let mut current_place = "AAA".to_owned();
    loop {
        let go_to = directions.get(&current_place).unwrap();
        current_place = instructions[0].aplay_instruction(go_to);
        steps += 1;
        instructions.rotate_left(1);
        if current_place == "ZZZ" {
            break;
        }
    }
    println!("{steps}");
}
