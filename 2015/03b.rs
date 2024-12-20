use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut sanas_pos = [(0, 0), (0, 0)];
    let mut who = 0;
    let mut visited = vec![sanas_pos[0]];
    for char in input[0].chars() {
        let pos = &mut sanas_pos[who];
        match char {
            '^' => pos.0 += 1,
            'v' => pos.0 -= 1,
            '>' => pos.1 += 1,
            '<' => pos.1 -= 1,
            _ => println!("Error"),
        }
        if !visited.contains(pos) {
            visited.push(*pos);
        }
        who = (who + 1) % 2;
    }
    println!("{}", visited.len());
}
