#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut simbols_index = Vec::new();
    let mut numbers_index = Vec::new();
    let mut schematic = Vec::new();
    for (y, row) in input.iter().enumerate() {
        let mut schematic_row = Vec::new();
        let mut number = Vec::new();
        for (x, char) in row.chars().enumerate() {
            if char.is_numeric() {
                number.push(char);
                schematic_row.push(Some(numbers_index.len()));
                // Continue loop so we don't push an uncomplete number
                continue;
            }
            if char == '*' {
                simbols_index.push((x, y));
            }
            if !number.is_empty() {
                numbers_index.push(number.iter().collect::<String>().parse::<usize>().unwrap());
                number = Vec::new();
            }
            schematic_row.push(None);
        }
        // Push number if we have one
        if !number.is_empty() {
            numbers_index.push(number.iter().collect::<String>().parse::<usize>().unwrap());
        }
        schematic.push(schematic_row);
    }
    let mut res = 0;
    for (x, y) in simbols_index {
        let mut local_res = Vec::new();
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                let x = x as isize + x_offset;
                let y = y as isize + y_offset;
                if x < 0
                    || y < 0
                    || x >= schematic[0].len() as isize
                    || y >= schematic.len() as isize
                {
                    continue;
                }
                if let Some(index) = schematic[y as usize][x as usize] {
                    local_res.push(index);
                }
            }
        }
        local_res.sort_unstable();
        local_res.dedup();
        if local_res.len() == 2 {
            let local_res = local_res.iter().fold(1, |acc, x| acc * numbers_index[*x]);
            res += local_res;
        }
    }
    println!("{res}");
}
