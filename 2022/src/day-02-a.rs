use helpers::load_input;

mod helpers;

#[derive(Debug)]
enum Outcome {
    Won,
    Draw,
    Lost,
}

impl Outcome {
    fn to_usize(&self) -> usize {
        match self {
            Outcome::Won => 6,
            Outcome::Draw => 3,
            Outcome::Lost => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
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
    fn to_usize(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn chaleng(&self, opponent: &RPS) -> Outcome {
        match (self, opponent) {
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
                Outcome::Lost
            }
            (s, o) if s == o => Outcome::Draw,
            _ => Outcome::Won,
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
        let outcome = my_play.chaleng(&opponent_play);
        score = score + my_play.to_usize() + outcome.to_usize();
    }
    println!("{:?}", score);
}
