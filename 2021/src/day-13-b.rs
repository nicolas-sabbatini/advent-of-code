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
    for line in &input {
        // Parse folds
        if line.len() > 11 {
            let mut line = line.split(' ').last().unwrap().split('=');

            let axis = line.next().unwrap();
            let row = line.next().unwrap().parse::<usize>().unwrap();

            match axis {
                "x" | "X" => folds.push((Axis::X, row)),
                "y" | "Y" => folds.push((Axis::Y, row)),
                _ => panic!("Panic!"),
            }
        // Parse dots
        } else if !line.is_empty() {
            let mut line = line.split(',');
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();
            paper_builder.push((x, y));
            max_x = max(max_x, x);
            max_y = max(max_y, y);
        }
    }
    // Build paper
    let mut paper: Vec<Vec<bool>> = vec![vec![false; max_x + 1]; max_y + 1];
    for cords in &paper_builder {
        paper[cords.1][cords.0] = true;
    }

    // Fold paper
    for f in folds {
        match f {
            (Axis::X, col) => {
                for x_offset in 0..(paper[0].len() - col) {
                    for row in &mut paper {
                        if x_offset > col {
                            break;
                        }
                        row[col - x_offset] = row[col - x_offset] || row[col + x_offset];
                        row[col + x_offset] = false;
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
    }
    print_paper(&paper);
}

fn print_paper(paper: &[Vec<bool>]) {
    for line in paper {
        let mut stored_empty = 0;
        let mut has_printed_line = false;
        for dot in line {
            if *dot {
                for _i in 0..stored_empty {
                    print!(" ");
                }
                print!("*");
                stored_empty = 0;
                has_printed_line = true;
            } else {
                stored_empty += 1;
            }
        }
        if has_printed_line {
            println!();
        }
    }
}
