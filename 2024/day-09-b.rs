use input_loader::load_input;

fn main() {
    let input = load_input();
    let mut disk = Vec::new();
    let mut is_free = false;
    let mut id = 0;
    for char in input[0].chars() {
        let num = char.to_string().parse::<usize>().unwrap();
        if is_free {
            disk.push((None, num));
        } else {
            disk.push((Some(id), num));
        }
        if !is_free {
            id += 1;
        }
        is_free = !is_free;
    }
    let mut back = disk.len() - 1;
    while back > 0 {
        while disk[back].0.is_none() && back > 0 {
            back -= 1;
        }
        for front in 0..back {
            if disk[front].0.is_some() || disk[front].1 < disk[back].1 {
                continue;
            }
            if disk[front].1 == disk[back].1 {
                disk.swap(back, front);
                break;
            }
            let help = disk.remove(back);
            disk.insert(back, (None, help.1));
            disk[front].1 -= help.1;
            disk.insert(front, help);
            break;
        }
        back -= 1;
    }
    let check_sum = disk
        .iter()
        .fold(Vec::new(), |mut acc, (id, size)| {
            for _ in 0..*size {
                acc.push(id.unwrap_or(0));
            }
            acc
        })
        .iter()
        .enumerate()
        .fold(0, |acc, (i, id)| acc + (i * id));
    println!("{check_sum}");
}
