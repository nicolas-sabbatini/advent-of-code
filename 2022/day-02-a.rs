use helpers::load_input;

mod helpers;

#[derive(Debug)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

#[derive(Debug, PartialEq)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Rps {
    fn from(ch: &str) -> Self {
        match ch {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissors,
            _ => panic!("unknown symbol"),
        }
    }
}

impl Rps {
    fn challenge(&self, opponent: &Rps) -> Outcome {
        if self == opponent {
            return Outcome::Draw;
        }
        if self.get_defeated_symbol() == *opponent {
            return Outcome::Won;
        }
        Outcome::Lost
    }

    fn get_defeated_symbol(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut score: usize = 0;
    for lines in input {
        let mut selection = lines.split(' ');
        let opponent_play = Rps::from(selection.next().unwrap());
        let my_play = Rps::from(selection.next().unwrap());
        let outcome = my_play.challenge(&opponent_play);
        score = score + my_play as usize + outcome as usize;
    }
    println!("{score:?}");
}
