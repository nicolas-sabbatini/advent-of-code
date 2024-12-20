#![allow(clippy::cast_sign_loss)]
use input_loader::load_input;

#[derive(Debug)]
struct Node {
    name: String,
    neighbors: Vec<(String, usize)>,
}

fn find_node(nodes: &[Node], name: &str) -> Option<usize> {
    for (i, node) in nodes.iter().enumerate() {
        if node.name == name {
            return Some(i);
        }
    }
    None
}

fn add_edge(nodes: &mut Vec<Node>, node_name_1: &str, node_name_2: &str, distance: usize) {
    match find_node(nodes, node_name_1) {
        Some(i) => nodes[i].neighbors.push((node_name_2.to_string(), distance)),
        None => {
            nodes.push(Node {
                name: node_name_1.to_string(),
                neighbors: vec![(node_name_2.to_string(), distance)],
            });
        }
    };
}

fn find_shortest_euler_path(graph: &[Node], start: usize) -> usize {
    let mut path: Vec<String> = Vec::new();
    let mut current_node = &graph[start];
    let mut current_distance = 0;
    path.push(current_node.name.clone());
    loop {
        let mut max_distance = 0;
        let mut next_node = None;
        for (neighbor, distance) in &current_node.neighbors {
            if !path.contains(neighbor) && distance > &max_distance {
                max_distance = *distance;
                next_node = Some(neighbor);
            }
        }
        match next_node {
            Some(neighbor) => {
                path.push(neighbor.clone());
                current_node = &graph[find_node(graph, neighbor).unwrap()];
                current_distance += max_distance;
            }
            None => break,
        }
    }
    current_distance
}

fn main() {
    // Load input
    let input = load_input();
    let mut graph: Vec<Node> = Vec::new();
    for line in input {
        let mut line = line.split(" = ");
        let mut path = line.next().unwrap().split(" to ");
        let node_name_1 = path.next().unwrap();
        let node_name_2 = path.next().unwrap();
        let distance = line.next().unwrap().parse::<i32>().unwrap();
        add_edge(&mut graph, node_name_1, node_name_2, distance as usize);
        add_edge(&mut graph, node_name_2, node_name_1, distance as usize);
    }
    let mut res = 0;
    for start in 0..graph.len() {
        let new = find_shortest_euler_path(&graph, start);
        if new > res {
            res = new;
        }
    }
    println!("{res}");
}
