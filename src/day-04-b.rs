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
    for line_index in 1..input.len() {
        match input[line_index].as_str() {
            // If is an empty line add a card
            "" => cards.push(Vec::new()),
            // If is a list of numbers
            _ => {
                // Get the current card
                let card_index = cards.len() - 1;
                // Add a row
                cards[card_index].push(Vec::new());
                // Get row index
                let row_index = cards[card_index].len() - 1;
                // Fill the row
                for num in input[line_index].split(" ") {
                    // If there are not a number
                    if num == "" {
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
    }
    let total_cards = cards.len();
    let mut done_cards = vec![false; total_cards];
    // Get the "random" numbers
    'rand_nums: for num in input[0].split(",") {
        // If there are not a number
        if num == "" {
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
                    // Mark
                    cards[*card_index][*row_index][*num_index].0 = true;
                    if check_card(&cards[*card_index]) {
                        done_cards[*card_index] = true;
                        if done_cards
                            .iter()
                            .fold(true, |prev, current| prev && *current)
                        {
                            println!("{}", calculate_card_point(&cards[*card_index], num));
                            break 'rand_nums;
                        }
                    }
                }
            }
        }
    }
}

fn check_card(card: &Vec<Vec<(bool, usize)>>) -> bool {
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
        for row in 0..rows {
            if card[row][col].0 {
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

fn calculate_card_point(card: &Vec<Vec<(bool, usize)>>, last_num: usize) -> usize {
    let mut points = 0;
    for row in card.iter() {
        points += row.iter().fold(
            0,
            |prev, (mark, value)| if !mark { *value + prev } else { prev },
        );
    }
    points * last_num
}
