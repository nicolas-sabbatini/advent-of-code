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
    let mut paths_buildier: Vec<(&str, usize)> = vec![(&"start", 0)];
    let mut need_to_add: Vec<usize> = vec![0];
    loop {
        let curren = need_to_add.pop().expect("Error!");
        let neighbors = graph.get(&paths_buildier[curren].0).expect("Error!");
        for n in neighbors {
            if (*n).to_string().chars().all(char::is_uppercase) {
                paths_buildier.push((n, curren));
                need_to_add.push(paths_buildier.len() - 1);
            } else if *n == "end" {
                paths_buildier.push((n, curren));
            } else {
                let prev_path = build_path(&paths_buildier, curren);
                if !prev_path.contains(n) {
                    paths_buildier.push((n, curren));
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
            paths.push(build_path(&paths_buildier, i));
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

fn build_path<'a>(p: &[(&'a str, usize)], index: usize) -> Vec<&'a str> {
    let mut path: Vec<&str> = Vec::new();
    let mut index = index;
    loop {
        let node = p[index].0;
        path.push(node);
        if p[index].0 == "start" {
            break;
        }
        index = p[index].1;
    }
    path.reverse();
    path
}
