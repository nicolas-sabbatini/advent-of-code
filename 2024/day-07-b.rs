#![allow(clippy::unnecessary_cast, clippy::cast_possible_truncation)]
use input_loader::load_input;

const MASK: usize = 0b11;

fn main() {
    let input = load_input();
    let mut res = 0;
    for line in input {
        let line = line.replace(':', "");
        let mut line = line.split_whitespace();
        let target = line.next().unwrap().parse::<usize>().unwrap();
        let combinators = line.map(|n| n.parse().unwrap()).collect::<Vec<usize>>();
        let convinations = (4 as usize).pow((combinators.len() - 1) as u32);
        'check_line: for operations in 0..convinations {
            let mut total = combinators[0];
            for (i, num) in combinators.iter().skip(1).enumerate() {
                let do_op = operations >> (i * 2);
                if do_op & MASK == 0b00 {
                    total += num;
                }
                if do_op & MASK == 0b01 {
                    total *= num;
                }
                if do_op & MASK == 0b10 {
                    let mut n1 = total.to_string();
                    n1.push_str(&num.to_string());
                    total = n1.parse::<usize>().unwrap();
                }
                if do_op & MASK == 0b11 {
                    continue 'check_line;
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
