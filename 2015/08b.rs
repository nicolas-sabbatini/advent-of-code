use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut char_on_string = 0;
    let mut char_in_re_encoded = 0;
    for line in input {
        char_on_string += line.len();
        let re_encoded = line.replace("\\\\", "||||").replace("\\\"", "||||");
        let hexe = re_encoded.matches("\\x").count();
        char_in_re_encoded += re_encoded.len() + hexe + 4;
    }
    println!("{}", char_in_re_encoded - char_on_string);
}
