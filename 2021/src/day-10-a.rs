#![allow(clippy::cast_possible_wrap)]
use core::panic;
use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    let mut res = 0;
    for line in &input {
        let mut chunk_open: Vec<char> = Vec::new();
        'line_chek: for chr in line.chars() {
            match chr {
                '(' | '[' | '{' | '<' => chunk_open.push(chr),
                _ => {
                    let prev = (chunk_open.len() - 1) as isize;
                    let error = if prev >= 0 {
                        check_char(chr, chunk_open[prev as usize])
                    } else {
                        check_char(chr, 'n')
                    };
                    if error > 0 {
                        res += error;
                        break 'line_chek;
                    }
                    chunk_open.remove(prev as usize);
                }
            }
        }
    }
    println!("{}", res);
}

fn check_char(c: char, target: char) -> usize {
    match (c, target) {
        (')', '(') | (']', '[') | ('}', '{') | ('>', '<') => 0,
        (')', _) => 3,
        (']', _) => 57,
        ('}', _) => 1197,
        ('>', _) => 25137,
        _ => panic!("Error!"),
    }
}
