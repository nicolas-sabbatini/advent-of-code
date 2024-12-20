use input_loader::load_input;

fn parse_table(input: &[String], carry_on: &mut usize) -> Vec<(usize, usize, usize)> {
    let mut table = Vec::new();
    for (i, line) in input[*carry_on..].iter().enumerate() {
        if line.is_empty() {
            *carry_on += i + 2;
            break;
        }
        let line = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        table.push((line[0], line[1], line[2]));
    }
    table
}

fn map_table(input: &[usize], table: &[(usize, usize, usize)]) -> Vec<usize> {
    let mut output = Vec::new();
    for &value in input {
        let mut found = false;
        for &(to, from, range) in table {
            if from <= value && value < from + range {
                let off_set = value - from;
                output.push(to + off_set);
                found = true;
                break;
            }
        }
        if !found {
            output.push(value);
        }
    }
    output
}

fn main() {
    let input = load_input();
    let seeds = input[0]
        .replace("seeds:", "")
        .trim()
        .split(' ')
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
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
    let mut locations = map
        .iter()
        .fold(seeds, |seeds, table| map_table(&seeds, table));
    locations.sort_unstable();
    println!("{}", locations[0]);
}
