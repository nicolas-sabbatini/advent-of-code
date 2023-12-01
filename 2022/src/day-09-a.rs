use helpers::load_input;

mod helpers;

fn move_right(
    map: &mut [Vec<bool>],
    hx: &mut usize,
    hy: &mut usize,
    tx: &mut usize,
    ty: &mut usize,
) {
    *hx += 1;
    if *tx + 1 < *hx {
        *tx += 1;
        if *ty != *hy {
            *ty = *hy;
        }
    }
    map[*ty][*tx] = true;
}

fn move_left(
    map: &mut [Vec<bool>],
    hx: &mut usize,
    hy: &mut usize,
    tx: &mut usize,
    ty: &mut usize,
) {
    *hx -= 1;
    if *tx - 1 > *hx {
        *tx -= 1;
        if *ty != *hy {
            *ty = *hy;
        }
    }
    map[*ty][*tx] = true;
}

fn move_up(map: &mut [Vec<bool>], hx: &mut usize, hy: &mut usize, tx: &mut usize, ty: &mut usize) {
    *hy -= 1;
    if *ty - 1 > *hy {
        *ty -= 1;
        if *tx != *hx {
            *tx = *hx;
        }
    }
    map[*ty][*tx] = true;
}

fn move_down(
    map: &mut [Vec<bool>],
    hx: &mut usize,
    hy: &mut usize,
    tx: &mut usize,
    ty: &mut usize,
) {
    *hy += 1;
    if *ty + 1 < *hy {
        *ty += 1;
        if *tx != *hx {
            *tx = *hx;
        }
    }
    map[*ty][*tx] = true;
}

fn main() {
    // Load input
    let input = load_input();
    let mut map = vec![vec![false; 5000]; 5000];
    let mut head_x = 2500;
    let mut head_y = 2500;
    let mut tail_x = 2500;
    let mut tail_y = 2500;

    for line in input {
        let command = line.split(' ').collect::<Vec<&str>>();
        for _i in 0..command[1].parse::<usize>().unwrap() {
            match *command.first().unwrap() {
                "R" => move_right(&mut map, &mut head_x, &mut head_y, &mut tail_x, &mut tail_y),
                "L" => move_left(&mut map, &mut head_x, &mut head_y, &mut tail_x, &mut tail_y),
                "U" => move_up(&mut map, &mut head_x, &mut head_y, &mut tail_x, &mut tail_y),
                "D" => move_down(&mut map, &mut head_x, &mut head_y, &mut tail_x, &mut tail_y),
                _ => panic!("Unkown direction"),
            }
        }
    }
    let res = map.iter().fold(0, |mut sum, row| {
        sum += row.iter().fold(0, |mut row_sum, cel| {
            if *cel {
                row_sum += 1;
            }
            row_sum
        });
        sum
    });
    println!("{res}");
}
