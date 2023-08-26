use input_loader::load_input;

const GRID_COLS: usize = 1000;
const GRID_ROWS: usize = 1000;
const GRID_SIZE: usize = GRID_COLS * GRID_ROWS;

fn from_x_y(x: usize, y: usize) -> usize {
    x + (y * GRID_COLS)
}

fn turn(grid: &mut [bool; GRID_SIZE], x1: usize, y1: usize, x2: usize, y2: usize, state: bool) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            let index = from_x_y(x, y);
            grid[index] = state;
        }
    }
}

fn toggle(grid: &mut [bool; GRID_SIZE], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            let index = from_x_y(x, y);
            grid[index] = !grid[index];
        }
    }
}

fn parse_cords(words: &mut std::str::SplitWhitespace) -> (usize, usize, usize, usize) {
    let mut coords = words.next().unwrap().split(',');
    let x1 = coords.next().unwrap().parse::<usize>().unwrap();
    let y1 = coords.next().unwrap().parse::<usize>().unwrap();
    coords = words.nth(1).unwrap().split(',');
    let x2 = coords.next().unwrap().parse::<usize>().unwrap();
    let y2 = coords.next().unwrap().parse::<usize>().unwrap();
    (x1, y1, x2, y2)
}

fn main() {
    // Load input
    let input = load_input();
    let mut grid = [false; GRID_SIZE];

    for line in input {
        let mut words = line.split_whitespace();
        let command = words.next().unwrap();

        match command {
            "turn" => {
                let state = match words.next().unwrap() {
                    "on" => true,
                    "off" => false,
                    _ => panic!("Invalid state"),
                };
                let (x1, y1, x2, y2) = parse_cords(&mut words);
                turn(&mut grid, x1, y1, x2, y2, state);
            }
            "toggle" => {
                let (x1, y1, x2, y2) = parse_cords(&mut words);
                toggle(&mut grid, x1, y1, x2, y2)
            }
            _ => panic!("Invalid command"),
        }
    }

    let res = grid.iter().fold(0, |acc, x| if *x { acc + 1 } else { acc });
    println!("{res}");
}
