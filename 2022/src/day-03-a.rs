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
    for line in input {
        let objets = line.chars().collect::<Vec<char>>();
        let mut first_compartment: Vec<char> = objets[0..objets.len() / 2].to_vec();
        let mut second_compartment: Vec<char> = objets[objets.len() / 2..].to_vec();
        first_compartment.sort();
        second_compartment.sort();
        let matching_objets = find_match_objet(first_compartment, second_compartment);
        let priority = matching_objets.iter().map(objet_to_priority).sum::<usize>();
        res += priority;
    }
    println!("{:?}", res);
}
