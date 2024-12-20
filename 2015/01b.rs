use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut floor = 0;
    let mut res = 1;
    for (i, char) in input[0].chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("Error"),
        }
        if floor == -1 {
            res = i + 1;
            break;
        }
    }
    println!("{res}");
}
