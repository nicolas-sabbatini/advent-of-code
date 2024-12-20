use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    // Parce input
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in &input {
        map.push(Vec::new());
        let pos = map.len() - 1;
        for height in line.chars() {
            let height = height.to_string().parse::<u8>().unwrap();
            map[pos].push(height);
        }
    }
    // Calculate low points
    let mut low_points: Vec<u8> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if ((x > 0 && h < &map[y][x - 1]) || x == 0)
                && ((x < row.len() - 1 && h < &map[y][x + 1]) || x == row.len() - 1)
                && ((y > 0 && h < &map[y - 1][x]) || y == 0)
                && ((y < map.len() - 1 && h < &map[y + 1][x]) || y == map.len() - 1)
            {
                low_points.push(*h);
            }
        }
    }

    let risk_sum: usize = low_points.iter().fold(0, |res, v| res + *v as usize + 1);
    println!("{risk_sum}");
}
