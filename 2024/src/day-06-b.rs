#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::similar_names
)]
use input_loader::load_input;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

impl Dir {
    fn rotate(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Unknown => panic!("Nada que hacer"),
        }
    }

    fn get_dir(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Unknown => panic!("Nada que hacer"),
        }
    }
}

enum Action<T> {
    Outside,
    Next(T),
    Wall,
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

fn find_next(
    map: &[Vec<char>],
    current_dir: Dir,
    cx: isize,
    cy: isize,
    ret_wall: bool,
) -> Action<(isize, isize, Dir)> {
    let mut current_dir = current_dir;
    loop {
        let (xdt, ydt) = current_dir.get_dir();
        let new_x = cx + xdt;
        let new_y = cy + ydt;
        if new_x < 0 || new_x >= map[0].len() as isize || new_y < 0 || new_y >= map.len() as isize {
            return Action::Outside;
        }
        if map[new_y as usize][new_x as usize] == '#' {
            if ret_wall {
                return Action::Wall;
            }
            current_dir = current_dir.rotate();
        } else {
            return Action::Next((new_x, new_y, current_dir));
        }
    }
}

fn phantom_walk(map: &[Vec<char>], (sx, sy, sdir): (isize, isize, Dir)) -> usize {
    let mut new_walk = vec![vec![Dir::Unknown; map[0].len()]; map.len()];
    let (mut cx, mut cy, mut cdir) = (sx, sy, sdir);
    while let Action::Next((new_cx, new_cy, new_current_dir)) = find_next(map, cdir, cx, cy, false)
    {
        cx = new_cx;
        cy = new_cy;
        cdir = new_current_dir;
        if cx == sx && cy == sy && cdir == sdir || new_walk[cy as usize][cx as usize] == cdir {
            return 1;
        }
        new_walk[cy as usize][cx as usize] = cdir;
    }
    0
}

fn walk(map: &[Vec<char>]) -> usize {
    let (cx, cy) = find_start(map);
    let mut obstacles = vec![vec![false; map[0].len()]; map.len()];
    obstacles[cy as usize][cx as usize] = true;
    let mut res = 0;
    let mut ncx: isize = cx;
    let mut ncy: isize = cy;
    let mut ndir = Dir::Up;
    while let Action::Next((new_cx, new_cy, new_dir)) = find_next(map, ndir, ncx, ncy, false) {
        ncx = new_cx;
        ncy = new_cy;
        ndir = new_dir;
        res += phantom_walk(
            &map.iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, cell)| {
                            if x as isize == ncx
                                && y as isize == ncy
                                && !obstacles[ncy as usize][ncx as usize]
                            {
                                return '#';
                            }
                            *cell
                        })
                        .collect()
                })
                .collect::<Vec<_>>(),
            (cx, cy, Dir::Up),
        );
        obstacles[ncy as usize][ncx as usize] = true;
    }
    res
}

fn main() {
    let input = load_input();
    let mut map = Vec::new();
    for line in input {
        let row = line.chars().collect::<Vec<_>>();
        map.push(row);
    }
    let res = walk(&map);
    println!("{res}");
}
