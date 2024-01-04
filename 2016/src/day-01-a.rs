use input_loader::load_input;

enum LookingAt {
    Up,
    Down,
    Left,
    Right,
}

impl LookingAt {
    fn rotate_left(&mut self) {
        *self = match self {
            LookingAt::Up => LookingAt::Left,
            LookingAt::Left => LookingAt::Down,
            LookingAt::Down => LookingAt::Right,
            LookingAt::Right => LookingAt::Up,
        };
    }

    fn rotate_right(&mut self) {
        *self = match self {
            LookingAt::Up => LookingAt::Right,
            LookingAt::Right => LookingAt::Down,
            LookingAt::Down => LookingAt::Left,
            LookingAt::Left => LookingAt::Up,
        };
    }

    fn move_forward(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            LookingAt::Up => (x, y + 1),
            LookingAt::Down => (x, y - 1),
            LookingAt::Left => (x - 1, y),
            LookingAt::Right => (x + 1, y),
        }
    }
}

fn main() {
    let input = load_input();
    let mut start_position = (0, 0);
    let mut looking_at = LookingAt::Up;
    for command in input[0].split(", ") {
        let command = command.chars().collect::<Vec<char>>();
        let dir = command[0];
        let steps = command[1..]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        match dir {
            'L' => looking_at.rotate_left(),
            'R' => looking_at.rotate_right(),
            _ => panic!("Unknown direction"),
        }
        for _ in 0..steps {
            start_position = looking_at.move_forward(start_position.0, start_position.1);
        }
    }
    println!("{}", start_position.0.abs() + start_position.1.abs());
}
