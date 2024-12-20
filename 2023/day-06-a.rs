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
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let top_distance = input[1]
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut result = 1;
    for (time, top_distance) in race_time.iter().zip(top_distance) {
        let distances = posible_distances(*time);
        result *= distances
            .iter()
            .filter(|distance| **distance > top_distance)
            .count();
    }

    println!("{result}");
}
