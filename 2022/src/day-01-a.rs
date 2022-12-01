use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    // Elf calories
    let mut elf_calories = vec![0];
    let mut last_elf = 0;
    for line in input {
        if line.is_empty() {
            elf_calories.push(0);
            last_elf += 1;
            continue;
        }
        let calories = line.parse::<usize>().unwrap();
        elf_calories[last_elf] += calories;
    }
    elf_calories.sort();
    println!("{:?}", elf_calories[last_elf]);
}
