use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut char_on_string = 0;
    let mut char_in_memory = 0;
    for line in input {
        char_on_string += line.len();
        let line = line
            .chars()
            .take(line.len() - 1)
            .skip(1)
            .collect::<String>()
            .replace("\\\\", "|")
            .replace("\\\"", "|");
        let hexe = line.matches("\\x").count();
        char_in_memory += line.len() - 3 * hexe;
    }
    println!("{}", char_on_string - char_in_memory);
}
