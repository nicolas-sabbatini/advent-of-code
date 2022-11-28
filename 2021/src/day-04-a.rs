use helpers::load_input;
use std::collections::HashMap;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    // create a place to store variables
    let mut cards = Vec::new();
    let mut indexs: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();
    // Iterate over input
    for line in &input[1..] {
        if let "" = line.as_str() {
            cards.push(Vec::new())
        } else {
            // Get the current card
            let card_index = cards.len() - 1;
            // Add a row
            cards[card_index].push(Vec::new());
            // Get row index
            let row_index = cards[card_index].len() - 1;
            // Fill the row
            for num in line.split(' ') {
                // If there are not a number
                if num.is_empty() {
                    continue;
                }
                // Parse number
                let num = num.parse::<usize>().unwrap();
                // Add "num" to the card
                cards[card_index][row_index].push((false, num));
                // Get position on row
                let num_index = cards[card_index][row_index].len() - 1;
                // Save num position on the Hash Map
                let pos = (card_index, row_index, num_index);
                match indexs.get_mut(&num) {
                    Some(saved_pos) => saved_pos.push(pos),
                    None => {
                        indexs.insert(num, vec![pos]);
                    }
                }
            }
        }
    }
    // Get the "random" numbers
    'rand_nums: for num in input[0].split(',') {
        // If there are not a number
        if num.is_empty() {
            continue;
        }
        // Parse number
        let num = num.parse::<usize>().unwrap();
        // See if the number is in the Hash Map
        match indexs.get(&num) {
            // If do not exist do nothing
            None => (),
            Some(pos) => {
                for (card_index, row_index, num_index) in pos.iter() {
                    cards[*card_index][*row_index][*num_index].0 = true;
                    if check_card(&cards[*card_index]) {
                        println!("{}", calculate_card_point(&cards[*card_index], num));
                        break 'rand_nums;
                    }
                }
            }
        }
    }
}

fn check_card(card: &[Vec<(bool, usize)>]) -> bool {
    // Get size
    let rows = card.len();
    let cols = card[0].len();
    // Check rows
    for row in card.iter() {
        let mut sum = 0;
        for (mark, _) in row.iter() {
            // If it was marked
            if *mark {
                sum += 1;
            // Do not make sense continue searching
            } else {
                break;
            }
        }
        // If there is a full row
        if sum == cols {
            // Return
            return true;
        }
    }
    // Check rows
    for col in 0..cols {
        let mut sum = 0;
        for row in card {
            if row[col].0 {
                sum += 1;
            } else {
                break;
            }
        }
        // if there is a full col
        if sum == rows {
            return true;
        }
    }
    false
}

fn calculate_card_point(card: &[Vec<(bool, usize)>], last_num: usize) -> usize {
    let mut points = 0;
    for row in card {
        points += row.iter().fold(
            0,
            |prev, (mark, value)| if !mark { *value + prev } else { prev },
        );
    }
    points * last_num
}
