use helpers::load_input;
use std::collections::HashMap;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in &input {
        let mut edge = line.split('-');
        let vertx1 = edge.next().unwrap();
        let vertx2 = edge.next().unwrap();
        add_edge(vertx1, vertx2, &mut graph);
        add_edge(vertx2, vertx1, &mut graph);
    }
    let mut paths_buildier: Vec<(&str, usize, bool)> = vec![(&"start", 0, false)];
    let mut need_to_add: Vec<usize> = vec![0];
    loop {
        let curren = need_to_add.pop().expect("Error!");
        let neighbors = graph.get(&paths_buildier[curren].0).expect("Error!");
        for n in neighbors {
            if (*n).to_string().chars().all(char::is_uppercase) {
                paths_buildier.push((n, curren, false));
                need_to_add.push(paths_buildier.len() - 1);
            } else if *n == "end" {
                paths_buildier.push((n, curren, false));
            } else {
                let (prev_path, twice) = build_path(&paths_buildier, curren);
                let already_in = prev_path.contains(n);
                if (!already_in || !twice) && *n != "start" {
                    paths_buildier.push((n, curren, already_in || twice));
                    need_to_add.push(paths_buildier.len() - 1);
                }
            }
        }
        if need_to_add.is_empty() {
            break;
        }
    }
    let mut paths: Vec<Vec<&str>> = Vec::new();
    for (i, p) in paths_buildier.iter().enumerate() {
        if p.0 == "end" {
            paths.push(build_path(&paths_buildier, i).0);
        }
    }
    println!("{}", paths.len());
}

fn add_edge<'a>(v1: &'a str, v2: &'a str, g: &mut HashMap<&'a str, Vec<&'a str>>) {
    match g.get_mut(v1) {
        None => {
            g.insert(v1, vec![v2]);
        }
        Some(neighbors) => {
            if !neighbors.contains(&v2) {
                neighbors.push(v2);
            }
        }
    }
}

fn build_path<'a>(p: &[(&'a str, usize, bool)], index: usize) -> (Vec<&'a str>, bool) {
    let mut path: Vec<&str> = Vec::new();
    let mut index = index;
    let mut twice = false;
    loop {
        let node = p[index].0;
        path.push(node);
        if p[index].0 == "start" {
            break;
        }
        twice = twice || p[index].2;
        index = p[index].1;
    }
    path.reverse();
    (path, twice)
}
