use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut possibles = 0;
    let mut rows = [Vec::new(), Vec::new(), Vec::new()];
    for line in input {
        let sides = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        rows[0].push(sides[0]);
        rows[1].push(sides[1]);
        rows[2].push(sides[2]);
    }
    for row in rows {
        for chunk in row.chunks_exact(3) {
            if chunk[0] + chunk[1] > chunk[2]
                && chunk[1] + chunk[2] > chunk[0]
                && chunk[2] + chunk[0] > chunk[1]
            {
                possibles += 1;
            }
        }
    }
    println!("{possibles}");
}
