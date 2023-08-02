use input_loader::load_input;

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];
const FORBIDDEN_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn main() {
    // Load input
    let input = load_input();
    let mut nice_strings = 0;
    'next_line: for string in input {
        let mut vowel_count = 0;
        let mut double_letter = false;
        let mut last_char = String::from("");
        for char in string.chars() {
            if VOWELS.contains(&char.to_string().as_str()) {
                vowel_count += 1;
            }
            if char.to_string() == last_char {
                double_letter = true;
            }
            if FORBIDDEN_STRINGS.contains(&format!("{}{}", last_char, char).as_str()) {
                continue 'next_line;
            }
            last_char = char.clone().to_string();
        }
        if vowel_count >= 3 && double_letter {
            nice_strings += 1;
        }
    }
    println!("{nice_strings}");
}
