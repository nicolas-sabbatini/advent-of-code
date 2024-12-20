use core::panic;
use std::cmp::Ordering;

use input_loader::load_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    Poker,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bet: usize,
}

impl Hand {
    fn get_value(&self) -> HandValue {
        let mut counts = [0; 13];
        for card in &self.cards {
            counts[(*card - 2) as usize] += 1;
        }
        counts.sort_unstable();
        counts.reverse();
        let cards = counts.chunks(2).next().unwrap();
        if cards[0] == 5 {
            HandValue::FiveOfAKind
        } else if cards[0] == 4 {
            HandValue::Poker
        } else if cards[0] == 3 && cards[1] == 2 {
            HandValue::FullHouse
        } else if cards[0] == 3 {
            HandValue::ThreeOfAKind
        } else if cards[0] == 2 && cards[1] == 2 {
            HandValue::TwoPair
        } else if cards[0] == 2 {
            HandValue::Pair
        } else {
            HandValue::HighCard
        }
    }

    fn cmp(&self, other_hand: &Hand) -> Ordering {
        match self.get_value().cmp(&other_hand.get_value()) {
            Ordering::Equal => {
                for (card, other_card) in self.cards.iter().zip(other_hand.cards.iter()) {
                    match card.cmp(other_card) {
                        Ordering::Equal => {}
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                    }
                }
                Ordering::Equal
            }
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

fn card_from_char(s: char) -> u8 {
    match s {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card: {}", s),
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut hands = Vec::new();
    for line in input {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let hand = Hand {
            cards: line[0].chars().map(card_from_char).collect::<Vec<u8>>(),
            bet: line[1].parse::<usize>().unwrap(),
        };
        hands.push(hand);
    }
    hands.sort_by(Hand::cmp);
    let res = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bet * (i + 1))
        .sum::<usize>();
    println!("{res}");
}
