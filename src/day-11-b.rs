use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    // Set the octopus
    let mut octopus: Vec<Vec<u8>> = Vec::new();
    let mut flash_already: Vec<Vec<bool>> = Vec::new();
    for line in input.iter() {
        octopus.push(Vec::new());
        flash_already.push(Vec::new());
        let row = octopus.len() - 1;
        for num in line.chars() {
            let num = num.to_string().parse::<u8>().unwrap();
            octopus[row].push(num);
            flash_already[row].push(false);
        }
    }
    let col = octopus.len() - 1;
    let row = octopus[0].len() - 1;
    let amount_octo = (col + 1) * (row + 1);
    // Simulate the steps
    let mut flashes = 0;
    let mut step = 0;
    loop {
        step += 1;
        let mut has_to_flash: Vec<(usize, usize)> = Vec::new();
        // Increase octopus energy
        for (y, row) in octopus.iter_mut().enumerate() {
            for (x, octo) in row.iter_mut().enumerate() {
                *octo += 1;
                if *octo > 9 {
                    has_to_flash.push((x, y));
                }
            }
        }
        // Flash
        loop {
            let some_pos = has_to_flash.pop();
            match some_pos {
                None => break,
                Some((x, y)) => {
                    if !flash_already[y][x] {
                        flashes += 1;
                        flash_already[y][x] = true;
                        for offset_y in -1..=1 {
                            for offset_x in -1..=1 {
                                // Skip if under or over flow
                                if (offset_y == -1 && y == 0)
                                    || (offset_x == -1 && x == 0)
                                    || (offset_y == 1 && y == col)
                                    || (offset_x == 1 && x == row)
                                {
                                    continue;
                                }
                                let new_y = (y as isize + offset_y) as usize;
                                let new_x = (x as isize + offset_x) as usize;
                                octopus[new_y][new_x] += 1;
                                if octopus[new_y][new_x] > 9 {
                                    has_to_flash.push((new_x, new_y));
                                }
                            }
                        }
                    }
                }
            }
        }
        // Restart
        let mut all_flashed = 0;
        for (y, row) in octopus.iter_mut().enumerate() {
            for (x, octo) in row.iter_mut().enumerate() {
                flash_already[y][x] = false;
                if *octo > 9 {
                    all_flashed += 1;
                    *octo = 0;
                }
            }
        }
        if all_flashed >= amount_octo {
            println!("{}", step);
            break;
        }
    }
}
