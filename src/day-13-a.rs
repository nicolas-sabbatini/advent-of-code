use helpers::load_input;
use std::cmp::max;

mod helpers;

enum Axis {
    X,
    Y,
}

fn main() {
    // Load input
    let input = load_input();
    let mut paper_builder: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(Axis, usize)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    // Parse input
    for line in input.iter() {
        // Parse folds
        if line.len() > 11 {
            let mut line = line.split(" ").last().unwrap().split("=");

            let axis = line.next().unwrap();
            let row = line.next().unwrap().parse::<usize>().unwrap();

            match axis {
                "x" | "X" => folds.push((Axis::X, row)),
                "y" | "Y" => folds.push((Axis::Y, row)),
                _ => panic!("Panic!"),
            }
        // Parse dots
        } else if line.len() > 0 {
            let mut line = line.split(",");
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();
            paper_builder.push((x, y));
            max_x = max(max_x, x);
            max_y = max(max_y, y);
        }
    }
    // Build paper
    let mut paper: Vec<Vec<bool>> = vec![vec![false; max_x + 1]; max_y + 1];
    for cords in paper_builder.iter() {
        paper[cords.1][cords.0] = true;
    }

    // Fold paper
    for f in folds {
        match f {
            (Axis::X, col) => {
                for x_offset in 0..(paper[0].len() - col) {
                    for y in 0..paper.len() {
                        if x_offset > col {
                            break;
                        }
                        paper[y][col - x_offset] =
                            paper[y][col - x_offset] || paper[y][col + x_offset];
                        paper[y][col + x_offset] = false;
                    }
                }
            }
            (Axis::Y, row) => {
                for y_offset in 0..(paper.len() - row) {
                    for x in 0..paper[0].len() {
                        if y_offset > row {
                            break;
                        }
                        paper[row - y_offset][x] =
                            paper[row - y_offset][x] || paper[row + y_offset][x];
                        paper[row + y_offset][x] = false;
                    }
                }
            }
        }
        break;
    }
    // Count dots
    let dots = paper.iter().fold(0, |sum, line| {
        sum + line
            .iter()
            .fold(0, |sum, dot| if *dot { sum + 1 } else { sum })
    });
    println!("{}", dots);
}
