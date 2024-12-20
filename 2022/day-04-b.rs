use helpers::load_input;

mod helpers;

fn overlap(first_range: &[usize], second_range: &[usize]) -> bool {
    if first_range[0] <= second_range[1] && first_range[1] >= second_range[0] {
        return true;
    }
    false
}

fn main() {
    // Load input
    let input = load_input();
    let mut overlap_ranges = 0;
    for line in input {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let first_range = ranges[0]
            .split('-')
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let second_range = ranges[1]
            .split('-')
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if overlap(&first_range, &second_range) {
            overlap_ranges += 1;
        }
    }
    println!("{overlap_ranges}");
}
