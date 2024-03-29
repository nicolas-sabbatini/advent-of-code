use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    let mut input_buffer = Vec::new();
    for line in input.lock().lines() {
        input_buffer.push(line.unwrap());
    }
    let size = input_buffer[0].len();

    let mut o2_gen: Vec<usize> = (0..input_buffer.len()).collect();
    let mut co2_scr = o2_gen.clone();

    o2_gen = filter_til_one(&input_buffer, o2_gen, size, &FilterMode::Common);
    co2_scr = filter_til_one(&input_buffer, co2_scr, size, &FilterMode::LessCommon);

    println!(
        "{}",
        binary_to_usize(&input_buffer[o2_gen[0]]) * binary_to_usize(&input_buffer[co2_scr[0]])
    );
}

enum FilterMode {
    Common,
    LessCommon,
}

fn filter_til_one(
    input: &[String],
    mut indexs: Vec<usize>,
    size: usize,
    mode: &FilterMode,
) -> Vec<usize> {
    for char_index in 0..size {
        let mut zero = 0;
        let mut one = 0;
        for index in &indexs {
            match input[*index].chars().nth(char_index).unwrap() {
                '0' => zero += 1,
                '1' => one += 1,
                _ => (),
            }
        }
        // Filter
        indexs = indexs
            .into_iter()
            .filter(|value| {
                let cha = input[*value].chars().nth(char_index).unwrap();
                match mode {
                    FilterMode::Common => {
                        if (one >= zero && cha == '1') || (one < zero && cha == '0') {
                            return true;
                        }
                        false
                    }
                    FilterMode::LessCommon => {
                        if (one < zero && cha == '1') || (one >= zero && cha == '0') {
                            return true;
                        }
                        false
                    }
                }
            })
            .collect::<Vec<usize>>();
        // Stop if is the last
        if indexs.len() <= 1 {
            break;
        }
    }
    indexs
}

fn binary_to_usize(num: &str) -> usize {
    usize::from_str_radix(num, 2).unwrap()
}
