#![allow(clippy::cast_possible_wrap)]
use std::collections::HashMap;

use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut antenas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let map_size = (input[0].len() as isize, input.len() as isize);
    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            match antenas.get_mut(&ch) {
                None => {
                    antenas.insert(ch, vec![(x as isize, y as isize); 1]);
                }
                Some(antena) => {
                    antena.push((x as isize, y as isize));
                }
            }
        }
    }
    let mut res = 0;
    let mut prev: HashMap<(isize, isize), bool> = HashMap::new();
    for (_, antenas_pos) in antenas {
        for a1 in &antenas_pos {
            for a2 in &antenas_pos {
                if a1 == a2 {
                    continue;
                }
                let dist_x = a1.0 - a2.0;
                let dist_y = a1.1 - a2.1;
                let op1 = (a1.0 - dist_x, a1.1 - dist_y);
                let op2 = (a1.0 + dist_x, a1.1 + dist_y);
                if op1 != *a2
                    && op1.0 >= 0
                    && op1.1 >= 0
                    && op1.0 < map_size.0
                    && op1.1 < map_size.1
                    && !prev.contains_key(&op1)
                {
                    res += 1;
                    prev.insert(op1, true);
                } else if op2 != *a2
                    && op2.0 >= 0
                    && op2.1 >= 0
                    && op2.0 < map_size.0
                    && op2.1 < map_size.1
                    && !prev.contains_key(&op2)
                {
                    res += 1;
                    prev.insert(op2, true);
                }
            }
        }
    }
    println!("{res}");
}
