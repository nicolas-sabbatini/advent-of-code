use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut possibles = 0;
    for line in input {
        let sides = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if sides[0] + sides[1] > sides[2]
            && sides[1] + sides[2] > sides[0]
            && sides[2] + sides[0] > sides[1]
        {
            possibles += 1;
        }
    }
    println!("{possibles}");
}
