use helpers::load_input;
use std::collections::HashMap;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    // Create map
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    // Generate list of coordinates
    for line in input.iter() {
        let mut coordinates = line.split(" -> ");
        let mut coor1 = coordinates.next().unwrap().split(",");
        let mut coor2 = coordinates.next().unwrap().split(",");
        let pair1 = (
            coor1.next().unwrap().parse::<usize>().unwrap(),
            coor1.next().unwrap().parse::<usize>().unwrap(),
        );
        let pair2 = (
            coor2.next().unwrap().parse::<usize>().unwrap(),
            coor2.next().unwrap().parse::<usize>().unwrap(),
        );
        for point in draw_line(pair1, pair2).iter() {
            match map.get_mut(&point) {
                Some(point) => *point += 1,
                None => {
                    map.insert(*point, 1);
                }
            }
        }
    }

    let res = map.iter().filter(|x| *x.1 > 1).count() as usize;

    println!("{}", res);
}

fn draw_line(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut line = vec![start];
    let mut trace = start;
    while trace.0 != end.0 || trace.1 != end.1 {
        trace.0 += (trace.0 < end.0) as usize;
        trace.0 -= (trace.0 > end.0) as usize;
        trace.1 += (trace.1 < end.1) as usize;
        trace.1 -= (trace.1 > end.1) as usize;
        line.push(trace);
    }
    line
}
