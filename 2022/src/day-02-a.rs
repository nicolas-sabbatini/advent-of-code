use helpers::load_input;

mod helpers;

#[derive(Debug)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
            _ => panic!("unknown simbol"),
        }
    }
}

impl RPS {
    fn chaleng(&self, opponent: &RPS) -> Outcome {
        if self == opponent {
            return Outcome::Draw;
        }
        if ((*self as usize - 1 + 2) % 3) + 1 == *opponent as usize {
            return Outcome::Won;
        }
        Outcome::Lost
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
        let outcome = my_play.chaleng(&opponent_play);
        score = score + my_play as usize + outcome as usize;
    }
    println!("{:?}", score);
}
