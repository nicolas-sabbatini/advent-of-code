use input_loader::load_input;

fn main() {
    // Load input
    let mut input = load_input();
    let sectret = input.pop().unwrap();
    let mut key = 0;
    loop {
        let hash = md5::compute(format!("{sectret}{key}"));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            println!("{key}");
            break;
        }
        key += 1;
    }
}
