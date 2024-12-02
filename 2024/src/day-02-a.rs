use std::cmp::Ordering;

use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut safe = 0;
    'line_loop: for line in input {
        let level = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let order = level[0].cmp(&level[1]);
        for win in level.windows(2) {
            if order == Ordering::Equal
                || win[0].cmp(&win[1]) != order
                || ((win[0] - win[1]).abs() > 3)
            {
                continue 'line_loop;
            }
        }
        safe += 1;
    }
    println!("{safe}");
}
