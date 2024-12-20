use input_loader::load_input;
use std::{cmp::Ordering, collections::VecDeque};

fn parse_table(input: &[String], carry_on: &mut usize) -> Vec<(isize, isize, isize)> {
    let mut table = Vec::new();
    for (i, line) in input[*carry_on..].iter().enumerate() {
        if line.is_empty() {
            *carry_on += i + 2;
            break;
        }
        let line = line
            .split(' ')
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        table.push((line[0], line[1], line[2]));
    }
    table
}

fn map_table(input: Vec<(isize, isize)>, table: &[(isize, isize, isize)]) -> Vec<(isize, isize)> {
    let mut output = Vec::new();
    let mut input = VecDeque::from_iter(input);
    while let Some((start, end)) = input.pop_front() {
        let mut find_something = false;
        if start >= end {
            continue;
        }
        for (oput_start, from_start, range) in table {
            let from_end = from_start + range;
            if end < *from_start || start > from_end {
                continue;
            }
            find_something = true;
            match (start.cmp(from_start), end.cmp(&from_end)) {
                (Ordering::Less, Ordering::Less | Ordering::Equal) => {
                    let off_set = end - from_start;
                    output.push((*oput_start, oput_start + off_set));
                    input.push_back((start, from_start - 1));
                    break;
                }
                (Ordering::Less, Ordering::Greater) => {
                    output.push((*oput_start, oput_start + range));
                    input.push_back((start, from_start - 1));
                    input.push_back((from_end + 1, end));
                    break;
                }
                (Ordering::Equal, Ordering::Less | Ordering::Equal) => {
                    let off_set = end - from_start;
                    output.push((*oput_start, oput_start + off_set));
                    break;
                }
                (Ordering::Equal, Ordering::Greater) => {
                    output.push((*oput_start, oput_start + range));
                    input.push_back((from_end + 1, end));
                    break;
                }
                (Ordering::Greater, Ordering::Less | Ordering::Equal) => {
                    let off_set_end = end - start;
                    let off_set_start = start - from_start;
                    output.push((
                        oput_start + off_set_start,
                        oput_start + off_set_start + off_set_end,
                    ));
                    break;
                }
                (Ordering::Greater, Ordering::Greater) => {
                    let off_set_start = start - from_start;
                    output.push((
                        oput_start + off_set_start,
                        oput_start + off_set_start + range,
                    ));
                    input.push_back((start, from_start - 1));
                    input.push_back((end + 1, from_end));
                    break;
                }
            }
        }
        if !find_something {
            output.push((start, end));
        }
        output.sort_unstable();
        output.dedup();
    }
    output
}

fn main() {
    let input = load_input();
    let seeds = input[0]
        .replace("seeds:", "")
        .trim()
        .split(' ')
        .map(|seed| seed.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .fold(Vec::new(), |mut seeds, chunk| {
            seeds.push((chunk[0], chunk[0] + chunk[1]));
            seeds
        });

    let mut carry_on = 3;
    let seed_to_soil = parse_table(&input, &mut carry_on);
    let soil_to_fertilizer = parse_table(&input, &mut carry_on);
    let fertilizer_to_water = parse_table(&input, &mut carry_on);
    let water_to_ligth = parse_table(&input, &mut carry_on);
    let light_to_temperature = parse_table(&input, &mut carry_on);
    let temperature_to_humidity = parse_table(&input, &mut carry_on);
    let humidity_to_location = parse_table(&input, &mut carry_on);
    let map = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_ligth,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];
    let locations = map
        .iter()
        .fold(seeds, |seeds, table| map_table(seeds, table));
    println!("{:?}", locations[0].0);
}
