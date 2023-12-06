use input_loader::load_input;

fn posible_distances(time: usize) -> Vec<usize> {
    let mut result = vec![0];
    for i in 1..=time {
        let velocity = i;
        let distance = velocity * (time - i);
        result.push(distance);
    }
    result
}

fn main() {
    let input = load_input();
    let race_time = input[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let top_distance = input[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distances = posible_distances(race_time);
    let result = distances
        .iter()
        .filter(|distance| **distance > top_distance)
        .count();
    println!("{result}");
}
