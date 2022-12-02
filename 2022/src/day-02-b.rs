use helpers::load_input;

mod helpers;

#[derive(Debug, PartialEq)]
enum Outcome {
    Won,
    Draw,
    Lost,
}

impl From<&str> for Outcome {
    fn from(ch: &str) -> Self {
        match ch {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Won,
            _ => panic!("unknown outcome"),
        }
    }
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
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
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

    fn select_simbol(&self, desire_outcome: &Outcome) -> RPS {
        if RPS::Rock.chaleng(self) == *desire_outcome {
            return RPS::Rock;
        }
        if RPS::Paper.chaleng(self) == *desire_outcome {
            return RPS::Paper;
        }
        if RPS::Scissors.chaleng(self) == *desire_outcome {
            return RPS::Scissors;
        }
        panic!("Unknown input");
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut score: usize = 0;
    for lines in input {
        let mut selection = lines.split(' ');
        let opponent_play = RPS::from(selection.next().unwrap());
        let desire_outcome = Outcome::from(selection.next().unwrap());
        let my_play = opponent_play.select_simbol(&desire_outcome);
        score = score + my_play.to_usize() + desire_outcome.to_usize();
    }
    println!("{:?}", score);
}
