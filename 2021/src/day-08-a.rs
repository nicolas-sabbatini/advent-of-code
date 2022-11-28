use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    // Aperences of 1, 4, 7, 8
    let mut aperences = 0;
    for line in input.iter() {
        let mut check = false;
        for wires in line.split(' ') {
            match wires {
                "|" => check = true,
                _ => {
                    if check {
                        let len = wires.len();
                        if len == 2 || len == 4 || len == 3 || len == 7 {
                            aperences += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", aperences);
}
