use input_loader::load_input;

struct Cordinates {
    x: usize,
    y: usize,
}

const KEYPAD: [[Option<char>; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None],
];

fn main() {
    let input = load_input();
    let mut res = String::new();
    let mut current_key = Cordinates { x: 0, y: 2 };
    for line in input {
        for char in line.chars() {
            match char {
                'U' if current_key.y > 0 && KEYPAD[current_key.y - 1][current_key.x].is_some() => {
                    current_key.y -= 1;
                }
                'D' if current_key.y < 4 && KEYPAD[current_key.y + 1][current_key.x].is_some() => {
                    current_key.y += 1;
                }
                'L' if current_key.x > 0 && KEYPAD[current_key.y][current_key.x - 1].is_some() => {
                    current_key.x -= 1;
                }
                'R' if current_key.x < 4 && KEYPAD[current_key.y][current_key.x + 1].is_some() => {
                    current_key.x += 1;
                }
                _ => {}
            }
        }
        res.push(KEYPAD[current_key.y][current_key.x].unwrap());
    }
    println!("{res}");
}
