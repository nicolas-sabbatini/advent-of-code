use input_loader::load_input;

#[derive(Debug, Default)]
struct Color {
    red: usize,
    blue: usize,
    green: usize,
}

impl Color {
    fn is_posible(&self, color: &Color) -> bool {
        self.red >= color.red && self.blue >= color.blue && self.green >= color.green
    }
}

fn main() {
    let input = load_input();
    let start_arragment = Color {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut res = 0;
    for line in input {
        let line = line.split(": ").collect::<Vec<&str>>();
        let id = line[0].replace("Game ", "").parse::<usize>().unwrap();
        let games = line[1]
            .split("; ")
            .map(|game| {
                game.split(", ")
                    .map(|mov| mov.split(' ').collect::<Vec<&str>>())
                    .collect::<Vec<_>>()
            })
            .map(|game| {
                let mut color = Color::default();
                for play in &game {
                    match *play.get(1).unwrap() {
                        "red" => color.red += play[0].parse::<usize>().unwrap(),
                        "blue" => color.blue += play[0].parse::<usize>().unwrap(),
                        "green" => color.green += play[0].parse::<usize>().unwrap(),
                        _ => panic!("Unknown color"),
                    }
                }
                color
            })
            .collect::<Vec<Color>>();
        if !games.iter().any(|game| !start_arragment.is_posible(game)) {
            res += id;
        }
    }
    println!("{res}");
}
