use helpers::load_input;
use std::{char, collections::HashMap};

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    let mut rules: Vec<((char, char), char)> = Vec::new();
    let mut polymer: HashMap<(char, char), u64> = HashMap::new();
    // Parce polymer
    let mut prev_char = ' ';
    for c in input[0].chars() {
        if prev_char != ' ' {
            match polymer.get_mut(&(prev_char, c)) {
                None => {
                    polymer.insert((prev_char, c), 1);
                }
                Some(v) => *v += 1,
            }
        }
        prev_char = c;
    }
    // Parse rules
    for rule in input.iter().skip(2) {
        let mut rule = rule.split(" -> ");
        let mut left = rule.next().unwrap().chars();
        let target1 = left.next().unwrap();
        let target2 = left.next().unwrap();

        let mut rigth = rule.next().unwrap().chars();
        let output = rigth.next().unwrap();

        rules.push(((target1, target2), output));
    }

    // Step
    for _step in 0..10 {
        let mut modifications: Vec<((char, char), u64)> = Vec::new();
        // Calculate new polymer
        for (targets, output) in rules.iter() {
            if let Some(v) = polymer.get(targets) {
                modifications.push(((targets.0, *output), *v));
                modifications.push(((*output, targets.1), *v));
            }
        }
        // generate new polymer
        polymer.drain();
        for (k, v) in modifications.iter() {
            match polymer.get_mut(k) {
                None => {
                    polymer.insert(*k, *v);
                }
                Some(prev_v) => *prev_v += v,
            }
        }
    }
    // Count appearances
    let mut appearances: Vec<u64> = polymer
        .iter()
        .fold(HashMap::<char, u64>::new(), |mut p, (k, v)| {
            match p.get_mut(&k.0) {
                None => {
                    p.insert(k.0, *v);
                }
                Some(prev_v) => *prev_v += *v,
            }
            match p.get_mut(&k.1) {
                None => {
                    p.insert(k.1, *v);
                }
                Some(prev_v) => *prev_v += *v,
            }
            p
        })
        .values()
        .fold(Vec::new(), |mut vec, v| {
            vec.push((*v / 2) + (*v % 2));
            vec
        });
    appearances.sort();
    let last = appearances.len() - 1;
    println!("{:?}", appearances[last] - appearances[0]);
}
