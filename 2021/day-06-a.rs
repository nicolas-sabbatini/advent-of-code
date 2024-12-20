use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    let mut fish_days = [0; 10];
    for num in input[0].split(',') {
        let num = num.parse::<usize>().unwrap();
        fish_days[num] += 1;
    }
    for _i in 0..80 {
        for i in 0..10 {
            let num_fish = fish_days[i];
            fish_days[i] = 0;
            if i == 0 {
                fish_days[7] += num_fish;
                fish_days[9] += num_fish;
            } else {
                fish_days[i - 1] += num_fish;
            }
        }
    }

    println!("{}", fish_days.iter().sum::<i32>());
}
