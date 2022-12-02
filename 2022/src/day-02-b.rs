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
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
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

impl From<usize> for RPS {
    fn from(input: usize) -> RPS {
        match input {
            1 => RPS::Rock,
            2 => RPS::Paper,
            3 => RPS::Scissors,
            _ => panic!("unknown simbol"),
        }
    }
}

fn select_correct_play(opponent_play: &RPS, desire_outcome: &Outcome) -> RPS {
    match desire_outcome {
        Outcome::Lost => RPS::from(((*opponent_play as usize - 1 + 2) % 3) + 1),
        Outcome::Draw => *opponent_play,
        Outcome::Won => RPS::from(((*opponent_play as usize - 1 + 1) % 3) + 1),
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
        let my_play = select_correct_play(&opponent_play, &desire_outcome);
        score = score + my_play as usize + desire_outcome as usize;
    }
    println!("{:?}", score);
}
