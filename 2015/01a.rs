use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut floor = 0;
    for char in input[0].chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("Error"),
        }
    }
    println!("{floor}");
}
