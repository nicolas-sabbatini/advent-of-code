use input_loader::load_input;

#[derive(Debug)]
struct Reindeer {
    flay_distance: usize,
    flay_time: usize,
    total_time: usize,
    total_distance: usize,
}

impl Reindeer {
    fn step(&mut self, time: usize) {
        let mut time = time;
        while time > 0 {
            if time >= self.total_time {
                self.total_distance += self.flay_distance * self.flay_time;
                time -= self.total_time;
            } else if time >= self.flay_time {
                self.total_distance += self.flay_distance * self.flay_time;
                time = 0;
            } else {
                self.total_distance += self.flay_distance * time;
                time = 0;
            }
        }
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut olympics: Vec<Reindeer> = Vec::new();
    for line in input {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let flay_distance = line[3].parse::<usize>().unwrap();
        let flay_time = line[6].parse::<usize>().unwrap();
        let rest_time = line[13].parse::<usize>().unwrap();
        olympics.push(Reindeer {
            flay_distance,
            flay_time,
            total_time: flay_time + rest_time,
            total_distance: 0,
        });
    }
    for reindeer in &mut olympics {
        reindeer.step(2503);
    }
    let res = olympics
        .iter()
        .max_by_key(|r| r.total_distance)
        .unwrap()
        .total_distance;
    println!("{res}");
}
