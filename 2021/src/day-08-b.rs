use helpers::load_input;
use std::{collections::HashMap, usize};

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    let mut signals = Vec::new();
    // Parce input
    for line in &input {
        let mut input = line.split(" | ");
        let unique_patterns = input.next().unwrap().split(' ').collect::<Vec<&str>>();
        let ouput = input.next().unwrap().split(' ').collect::<Vec<&str>>();
        signals.push((unique_patterns, ouput));
    }
    // Get a decode pattern
    /*
          aaaa    ....    aaaa    aaaa    ....
         b    c  .    c  .    c  .    c  b    c
         b    c  .    c  .    c  .    c  b    c
          ....    ....    dddd    dddd    dddd
         e    f  .    f  e    .  .    f  .    f
         e    f  .    f  e    .  .    f  .    f
          gggg    ....    gggg    gggg    ....

          aaaa    aaaa    aaaa    aaaa    aaaa
         b    .  b    .  .    c  b    c  b    c
         b    .  b    .  .    c  b    c  b    c
          dddd    dddd    ....    dddd    dddd
         .    f  e    f  .    f  e    f  .    f
         .    f  e    f  .    f  e    f  .    f
          gggg    gggg    ....    gggg    gggg


    num/segment| a | b | c | d | e | f | g |
             -------------------------------
             0 | x | x | x |   | x | x | x |
             -------------------------------
             1 |   |   | x |   |   | x |   |
             -------------------------------
             2 | x |   | x | x | x |   | x |
             -------------------------------
             3 | x |   | x | x |   | x | x |
             -------------------------------
             4 |   | x | x | x |   | x |   |
             -------------------------------
             5 | x | x |   | x |   | x | x |
             -------------------------------
             6 | x | x |   | x | x | x | x |
             -------------------------------
             7 | x |   | x |   |   | x |   |
             -------------------------------
             8 | x | x | x | x | x | x | x |
             -------------------------------
             9 | x | x | x | x |   | x | x |
             -------------------------------
          total| 8 | 6 | 8 | 7 | 4 | 9 | 7 |
             -------------------------------
        e = only appears 4 times
        b = only appears 6 times
        f = only appears 9 times
        c = appears in the number 4 and appears 8 times
        a = DON'T appears in the number 4 and appears 8 times - done
        d = appears in the number 4 and appears 7 times
        g = DON'T appears in the number 4 and appears 7 times
    */
    let mut value = 0;
    // Decode each signal
    for signal in &signals {
        let mut wires_aperance: HashMap<char, usize> = HashMap::new();
        let mut is_four: Vec<char> = Vec::new();
        let mut decoded_wires: HashMap<char, u8> = HashMap::new();
        // Count the appearances of the wires
        for wires in &signal.0 {
            for wire in wires.chars() {
                match wires_aperance.get_mut(&wire) {
                    None => {
                        wires_aperance.insert(wire, 1);
                    }
                    Some(amount) => *amount += 1,
                }
            }
            // If is wired to 4
            if wires.len() == 4 {
                is_four = wires.chars().collect::<Vec<char>>();
                is_four.sort_unstable();
            }
        }
        // Decode patterns
        for (k, v) in &wires_aperance {
            if *v == 8 && !is_four.contains(k) {
                decoded_wires.insert(*k, 0b0000_0001);
            } else if *v == 6 {
                decoded_wires.insert(*k, 0b0000_0010);
            } else if *v == 8 {
                decoded_wires.insert(*k, 0b0000_0100);
            } else if *v == 7 && is_four.contains(k) {
                decoded_wires.insert(*k, 0b0000_1000);
            } else if *v == 4 {
                decoded_wires.insert(*k, 0b0001_0000);
            } else if *v == 9 {
                decoded_wires.insert(*k, 0b0010_0000);
            } else if *v == 7 {
                decoded_wires.insert(*k, 0b0100_0000);
            }
        }
        // Decode 4 digit code
        let mut num = 0;
        for (offset, wires) in signal.1.iter().rev().enumerate() {
            // Generate a mask for the number
            let mut mask: u8 = 0;
            for wire in wires.chars() {
                let value = decoded_wires.get(&wire);
                if let Some(v) = value {
                    mask |= v;
                }
            }
            // decode and offset number
            num += mask_to_num(mask) * usize::pow(10, u32::try_from(offset).unwrap());
        }
        // add decoded number to result
        value += num;
    }
    // print result
    println!("{value}");
}

fn mask_to_num(target: u8) -> usize {
    match target {
        0b0111_0111 => 0,
        0b0010_0100 => 1,
        0b0101_1101 => 2,
        0b0110_1101 => 3,
        0b0010_1110 => 4,
        0b0110_1011 => 5,
        0b0111_1011 => 6,
        0b0010_0101 => 7,
        0b0111_1111 => 8,
        0b0110_1111 => 9,
        _ => panic!("Error!"),
    }
}
