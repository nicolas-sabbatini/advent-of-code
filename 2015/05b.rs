use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut nice_strings = 0;
    'next_line: for string in input {
        let mut repeat_one_letter_before = false;
        let mut pair_repeat = false;
        let mut pairs = Vec::new();
        let mut previous_char = String::new();
        let mut char_test = String::new();
        for char in string.chars() {
            if char.to_string() == char_test {
                repeat_one_letter_before = true;
            }
            char_test = previous_char.clone();
            let pair = format!("{previous_char}{char}");
            let pair_len = pairs.len();
            if pair_len > 1 && pairs[0..(pair_len - 1)].contains(&pair) && !pair_repeat {
                pair_repeat = true;
            }
            pairs.push(pair);
            previous_char = char.clone().to_string();
            if repeat_one_letter_before && pair_repeat {
                nice_strings += 1;
                continue 'next_line;
            }
        }
    }
    println!("{nice_strings}");
}
