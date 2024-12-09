#![allow(clippy::unnecessary_cast, clippy::cast_possible_truncation)]
use input_loader::load_input;

const MASK: usize = 1;

fn main() {
    let input = load_input();
    let mut res = 0;
    for line in input {
        let line = line.replace(':', "");
        let mut line = line.split_whitespace();
        let target = line.next().unwrap().parse::<usize>().unwrap();
        let combinators = line.map(|n| n.parse().unwrap()).collect::<Vec<usize>>();
        let convinations = (2 as usize).pow((combinators.len() - 1) as u32);
        'check_line: for operations in 0..convinations {
            let mut total = combinators[0];
            for (i, num) in combinators.iter().skip(1).enumerate() {
                if (operations >> i) & MASK == 1 {
                    total += num;
                } else {
                    total *= num;
                }
            }
            if total == target {
                res += target;
                break 'check_line;
            }
        }
    }
    println!("{res}");
}
