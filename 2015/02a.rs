use input_loader::load_input;

fn main() {
    // Load input
    let input = load_input();
    let mut total = 0;
    for line in input {
        let lwh = line.split('x').collect::<Vec<&str>>();
        let l = lwh[0].parse::<usize>().unwrap();
        let w = lwh[1].parse::<usize>().unwrap();
        let h = lwh[2].parse::<usize>().unwrap();
        let mut sides = [l * w, w * h, h * l];
        sides.sort_unstable();
        total += sides[0] + sides.iter().sum::<usize>() * 2;
    }
    println!("{total}");
}
