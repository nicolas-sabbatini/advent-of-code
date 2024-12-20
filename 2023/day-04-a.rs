use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut res = 0;
    for line in input {
        let line = line.split(": ").collect::<Vec<&str>>();
        let line = line[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = line[0]
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let card_numbers = line[1]
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut local_res = 0;
        for winning_number in winning_numbers {
            if card_numbers.contains(&winning_number) {
                if local_res == 0 {
                    local_res += 1;
                } else {
                    local_res *= 2;
                }
            }
        }
        res += local_res;
    }
    println!("{res:?}");
}
