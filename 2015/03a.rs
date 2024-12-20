use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut pos = (0, 0);
    let mut visited = vec![pos];
    for char in input[0].chars() {
        match char {
            '^' => pos.0 += 1,
            'v' => pos.0 -= 1,
            '>' => pos.1 += 1,
            '<' => pos.1 -= 1,
            _ => println!("Error"),
        }
        if !visited.contains(&pos) {
            visited.push(pos);
        }
    }
    println!("{}", visited.len());
}
