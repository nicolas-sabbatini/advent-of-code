use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut res = Vec::new();
    for line in &input {
        let line = line
            .to_lowercase()
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.to_string().parse::<usize>().is_ok() {
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
