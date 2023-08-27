use input_loader::load_input;

fn iterate_secuence(input: &[char]) -> Vec<char> {
    let mut new_vec = Vec::new();
    let mut prev = input[0];
    let mut appears = 1;
    for c in input.iter().skip(1) {
        if *c != prev {
            let mut to_concat = appears.to_string().chars().collect::<Vec<char>>();
            new_vec.append(&mut to_concat);
            new_vec.push(prev);
            prev = *c;
            appears = 1;
            continue;
        }
        appears += 1;
    }
    let mut to_concat = appears.to_string().chars().collect::<Vec<char>>();
    new_vec.append(&mut to_concat);
    new_vec.push(prev);
    new_vec
}

fn main() {
    // Load input
    let input = load_input();
    let mut res = input[0].chars().collect::<Vec<char>>();
    for _i in 0..50 {
        res = iterate_secuence(&res);
    }
    println!("{}", res.len());
}
