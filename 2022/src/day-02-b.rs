use helpers::load_input;

mod helpers;

#[derive(Debug)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Rps {
    fn from(ch: &str) -> Self {
        match ch {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("unknown symbol"),
        }
    }
}

impl Rps {
    fn get_defeated_symbol(self) -> Rps {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    fn get_winner_symbol(self) -> Rps {
        match self {
            Rps::Scissors => Rps::Rock,
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
        }
    }
}

fn select_desired_play(opponent_play: Rps, desire_outcome: &Outcome) -> Rps {
    match desire_outcome {
        Outcome::Lost => opponent_play.get_defeated_symbol(),
        Outcome::Draw => opponent_play,
        Outcome::Won => opponent_play.get_winner_symbol(),
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut score: usize = 0;
    for lines in input {
        let mut selection = lines.split(' ');
        let opponent_play = Rps::from(selection.next().unwrap());
        let desire_outcome = Outcome::from(selection.next().unwrap());
        let my_play = select_desired_play(opponent_play, &desire_outcome);
        score = score + my_play as usize + desire_outcome as usize;
    }
    println!("{score:?}");
}
