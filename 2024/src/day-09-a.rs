use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut disk = Vec::new();
    let mut is_free = false;
    let mut id = 0;
    for char in input[0].chars() {
        let num = char.to_string().parse::<usize>().unwrap();
        for _ in 0..num {
            if is_free {
                disk.push(None);
            } else {
                disk.push(Some(id));
            }
        }
        if !is_free {
            id += 1;
        }
        is_free = !is_free;
    }
    let mut front = 0;
    let mut back = disk.len() - 1;
    while front < back {
        while disk[front].is_some() && front < disk.len() - 1 {
            front += 1;
        }
        while disk[back].is_none() && back > 0 {
            back -= 1;
        }
        if front < back {
            disk.swap(back, front);
        }
    }
    let check_sum = disk
        .iter()
        .enumerate()
        .fold(0, |acc, (i, id)| acc + (i * id.unwrap_or_else(|| 0)));
    println!("{check_sum}");
}
