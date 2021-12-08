use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    let mut crab_positions = Vec::new();
    // Parse crab positions
    for num in input[0].split(",") {
        let num = num.parse::<isize>().unwrap();
        crab_positions.push(num);
    }
    // Sort crabs
    crab_positions.sort();
    // Calculate the min cost;
    let mut res = isize::MAX;
    for i in crab_positions[0]..crab_positions[crab_positions.len() - 1] {
        let mut cost = 0;
        for crab_pos in crab_positions.iter() {
            // Use Gauss
            let steps = (crab_pos - i).abs();
            cost += (steps * (steps + 1)) / 2;
        }
        res = if cost < res { cost } else { res }
    }
    println!("{}", res);
}
