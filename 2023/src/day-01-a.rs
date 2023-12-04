use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut res = Vec::new();
    for line in &input {
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c);
            }
        }
        res.push(
            [numbers[0], numbers[numbers.len() - 1]]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .expect("Not a number"),
        );
    }
    println!("{:?}", res.iter().sum::<usize>());
}
