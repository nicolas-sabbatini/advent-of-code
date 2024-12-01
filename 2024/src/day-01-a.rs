use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut input_1 = Vec::new();
    let mut input_2 = Vec::new();
    for line in input {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap());
        input_1.push(nums.next().unwrap());
        input_2.push(nums.next().unwrap());
    }
    input_1.sort_unstable();
    input_2.sort_unstable();
    let acc = input_1.iter().zip(&input_2).fold(0, |acc, (n1, n2)| {
        let diff = (n1 - n2).abs();
        acc + diff
    });
    println!("{acc}");
}
