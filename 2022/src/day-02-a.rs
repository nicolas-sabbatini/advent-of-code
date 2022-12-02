use helpers::load_input;

mod helpers;

#[derive(Debug)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

#[derive(Debug, PartialEq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for RPS {
    fn from(ch: &str) -> Self {
        match ch {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("unknown symbol"),
        }
    }
}

impl RPS {
    fn challenge(&self, opponent: &RPS) -> Outcome {
        if self == opponent {
            return Outcome::Draw;
        }
        if self.get_defeated_symbol() == *opponent {
            return Outcome::Won;
        }
        Outcome::Lost
    }

    fn get_defeated_symbol(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut score: usize = 0;
    for lines in input {
        let mut selection = lines.split(' ');
        let opponent_play = RPS::from(selection.next().unwrap());
        let my_play = RPS::from(selection.next().unwrap());
        let outcome = my_play.challenge(&opponent_play);
        score = score + my_play as usize + outcome as usize;
    }
    println!("{:?}", score);
}
