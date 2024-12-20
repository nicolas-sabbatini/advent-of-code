use std::cmp::Ordering;

use input_loader::load_input;

fn calculate_order(lvl: &[isize]) -> Ordering {
    let mut gt = 0;
    let mut lt = 0;
    for win in lvl.windows(2) {
        match win[0].cmp(&win[1]) {
            Ordering::Less => lt += 1,
            Ordering::Greater => gt += 1,
            Ordering::Equal => (),
        };
    }
    if gt > lt {
        return Ordering::Greater;
    }
    Ordering::Less
}

fn check_level(lvl: &[isize]) -> bool {
    let order = calculate_order(lvl);
    for win in lvl.windows(2) {
        if win[0].cmp(&win[1]) != order || ((win[0] - win[1]).abs() > 3) {
            return false;
        }
    }
    true
}

fn main() {
    let input = load_input();
    let mut safe = 0;
    for line in input {
        let level = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let mut ok = check_level(&level);
        let mut i = 0;
        while !ok && i < level.len() {
            let mut lvl = level.clone();
            lvl.remove(i);
            ok = check_level(&lvl);
            i += 1;
        }
        safe += usize::from(ok);
    }
    println!("{safe}");
}
