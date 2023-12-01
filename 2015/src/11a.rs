use input_loader::load_input;

const OFFSET: u32 = 97;
const ALPHABET_SIZE: u32 = 26;
const ILLEGAL_CHARS: [u32; 3] = [
    'i' as u32 - OFFSET,
    'o' as u32 - OFFSET,
    'l' as u32 - OFFSET,
];

fn create_new_password(mut old: Vec<u32>) -> Vec<u32> {
    let mut illegal_chars_pos = Vec::new();
    for ilegal in &ILLEGAL_CHARS {
        if let Some(i) = old.iter().position(|&x| x == *ilegal) {
            illegal_chars_pos.push(i);
        }
    }

    if !illegal_chars_pos.is_empty() {
        illegal_chars_pos.sort();
        old[illegal_chars_pos[0]] += 1;
        for j in old.iter_mut().skip(illegal_chars_pos[0] + 1) {
            *j = 0;
        }
        return old;
    }

    old.reverse();
    let mut new = Vec::new();
    let mut carry_on = true;
    for c in old {
        if carry_on {
            let new_c = (c + 1) % ALPHABET_SIZE;
            new.push(new_c);
            if new_c != 0 {
                carry_on = false;
            }
        } else {
            new.push(c);
        }
    }
    new.reverse();
    new
}

fn is_valid(password: &[u32]) -> bool {
    for c in password.iter() {
        if ILLEGAL_CHARS.contains(c) {
            return false;
        }
    }
    let mut has_increasing = false;
    for c in password.windows(3) {
        if c[0] + 1 == c[1] && c[1] + 1 == c[2] {
            has_increasing = true;
            break;
        }
    }
    let mut pairs = 0;
    let mut last_pair = 0;
    for (i, c) in password.windows(2).enumerate() {
        if c[0] == c[1] && i as isize - 1 != last_pair {
            pairs += 1;
            last_pair = i as isize;
        }
    }
    has_increasing && pairs >= 2
}

fn next_password(old: Vec<u32>) -> Vec<u32> {
    let mut new = create_new_password(old);
    while !is_valid(&new) {
        new = create_new_password(new);
    }
    new
}

fn main() {
    // Load input
    let input = load_input();
    let old_password = input[0]
        .chars()
        .map(|x| x as u32 - OFFSET)
        .collect::<Vec<u32>>();
    println!("{}", input[0]);
    let new_password = next_password(old_password)
        .iter()
        .map(|x| (x + OFFSET) as u8 as char)
        .collect::<String>();
    println!("{new_password}");
}
