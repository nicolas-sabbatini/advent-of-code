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
    let mut acc = Vec::new();
    for n1 in &input_1 {
        let mut count = 0;
        for n2 in &input_2 {
            match n2.cmp(n1) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => count += 1,
                std::cmp::Ordering::Greater => break,
            }
        }
        acc.push(n1 * count);
    }
    println!("{}", acc.iter().sum::<isize>());
}
