#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
use input_loader::load_input;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn rotate(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn get_dir(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

fn find_start(map: &[Vec<char>]) -> (isize, isize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cel) in row.iter().enumerate() {
            if *cel == '^' {
                return (x as isize, y as isize);
            }
        }
    }
    panic!("No puede encontrar el inicio!");
}

fn walk(map: &[Vec<char>], res: &mut [Vec<bool>]) {
    let mut current_dir = Dir::Up;
    let (mut cx, mut cy) = find_start(map);
    'main: loop {
        res[cy as usize][cx as usize] = true;
        loop {
            let (xdt, ydt) = current_dir.get_dir();
            let new_x = cx + xdt;
            let new_y = cy + ydt;
            if new_x < 0
                || new_x >= map[0].len() as isize
                || new_y < 0
                || new_y >= map.len() as isize
            {
                break 'main;
            }
            if map[new_y as usize][new_x as usize] == '#' {
                current_dir = current_dir.rotate();
            } else {
                cx = new_x;
                cy = new_y;
                break;
            }
        }
    }
}

fn main() {
    let input = load_input();
    let mut map = Vec::new();
    let mut visited = Vec::new();
    for line in input {
        let row = line.chars().collect::<Vec<_>>();
        visited.push(vec![false; row.len()]);
        map.push(row);
    }
    walk(&map, &mut visited);
    let res = visited.iter().fold(0, |acc, row| {
        let row_count = row.iter().fold(0, |acc, cell| acc + usize::from(*cell));
        acc + row_count
    });
    println!("{res}");
}
