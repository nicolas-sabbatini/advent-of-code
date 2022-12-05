use helpers::load_input;

mod helpers;

const UPPERCASE_OFFSET: usize = 38;
const LOWERCASE_OFFSET: usize = 96;

fn find_match_objet(a_compartment: Vec<char>, b_compartment: Vec<char>) -> Vec<char> {
    let mut a_i = 0;
    let mut b_i = 0;
    let mut matching_objets: Vec<char> = Vec::new();
    while a_i < a_compartment.len() && b_i < b_compartment.len() {
        if a_compartment[a_i] < b_compartment[b_i] && a_i < a_compartment.len() {
            a_i += 1;
            continue;
        }
        if a_compartment[a_i] == b_compartment[b_i] {
            matching_objets.push(a_compartment[a_i]);
        }
        b_i += 1;
    }
    matching_objets.dedup();
    matching_objets
}

fn objet_to_priority(objet: &char) -> usize {
    if *objet < 'a' {
        return *objet as usize - UPPERCASE_OFFSET;
    }
    *objet as usize - LOWERCASE_OFFSET
}

fn main() {
    let input = load_input();
    let mut res = 0;
    let input_size = input.len();
    let mut lines = input.iter();
    let mut i = 0;
    while i < input_size {
        let mut first_rucksacks: Vec<char> = lines.next().unwrap().chars().collect();
        let mut second_rucksacks: Vec<char> = lines.next().unwrap().chars().collect();
        let mut third_rucksacks: Vec<char> = lines.next().unwrap().chars().collect();
        first_rucksacks.sort();
        second_rucksacks.sort();
        third_rucksacks.sort();
        let matching_objets = find_match_objet(first_rucksacks, second_rucksacks);
        let matching_objets = find_match_objet(matching_objets, third_rucksacks);
        let priority = matching_objets.iter().map(objet_to_priority).sum::<usize>();
        println!("{:?}", priority);
        res += priority;
        i += 3;
    }
    println!("{:?}", res);
}
