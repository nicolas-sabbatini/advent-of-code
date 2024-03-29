use input_loader::load_input;

#[derive(Debug)]
struct Reindeer {
    flay_distance: usize,
    flay_time: usize,
    total_time: usize,
    total_distance: usize,
    elapsed_time: usize,
    points: usize,
}

impl Reindeer {
    fn step(&mut self) {
        if (self.elapsed_time % self.total_time) < self.flay_time {
            self.total_distance += self.flay_distance;
        }
        self.elapsed_time += 1;
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
            elapsed_time: 0,
            points: 0,
        });
    }
    for _ in 0..2503 {
        for reindeer in &mut olympics {
            reindeer.step();
        }
        olympics.sort_by_key(|r| r.total_distance);
        let max_dist = olympics.last_mut().unwrap().total_distance;
        for reindeer in &mut olympics {
            if reindeer.total_distance == max_dist {
                reindeer.points += 1;
            }
        }
    }
    let res = olympics.iter().max_by_key(|r| r.points).unwrap().points;
    println!("{res}");
}
