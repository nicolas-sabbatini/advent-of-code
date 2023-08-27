use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let parced_input = input[0].replace(['{', '}', '[', ']', '(', ')', ':', ','], " ");
    let mut res = 0;
    for st in parced_input.split_whitespace() {
        if let Ok(num) = st.parse::<i32>() {
            res += num;
        }
    }
    println!("{res}");
}
