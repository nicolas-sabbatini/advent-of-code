use input_loader::load_input;

#[derive(Debug, Default)]
struct Color {
    red: usize,
    blue: usize,
    green: usize,
}

impl Color {
    fn power(&self) -> usize {
        self.red * self.blue * self.green
    }
}

fn main() {
    let input = load_input();
    let mut games = Vec::new();
    for line in input {
        let line = line.split(": ").collect::<Vec<&str>>();
        let min_posible_combination = line[1]
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
            .fold(Color::default(), |acc, color| Color {
                red: acc.red.max(color.red),
                blue: acc.blue.max(color.blue),
                green: acc.green.max(color.green),
            });
        games.push(min_posible_combination.power());
    }
    println!("{:?}", games.iter().sum::<usize>());
}
