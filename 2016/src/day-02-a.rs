use input_loader::load_input;

struct Cordinates {
    x: usize,
    y: usize,
}

const KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

fn main() {
    let input = load_input();
    let mut res = String::new();
    let mut current_key = Cordinates { x: 1, y: 1 };
    for line in input {
        for char in line.chars() {
            match char {
                'U' if current_key.y > 0 => {
                    current_key.y -= 1;
                }
                'D' if current_key.y < 2 => {
                    current_key.y += 1;
                }
                'L' if current_key.x > 0 => {
                    current_key.x -= 1;
                }
                'R' if current_key.x < 2 => {
                    current_key.x += 1;
                }
                _ => {}
            }
        }
        res.push(KEYPAD[current_key.y][current_key.x]);
    }
    println!("{res}");
}
