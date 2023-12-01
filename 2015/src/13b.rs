use input_loader::load_input;

#[derive(Debug)]
struct Node {
    name: String,
    neighbors: Vec<(String, isize)>,
}

fn find_node(nodes: &[Node], name: &str) -> Option<usize> {
    for (i, node) in nodes.iter().enumerate() {
        if node.name == name {
            return Some(i);
        }
    }
    None
}

fn add_edge(nodes: &mut Vec<Node>, node_name_1: &str, node_name_2: &str, distance: isize) {
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

fn calculate_options(option: Vec<Vec<String>>, graph: &[Node]) -> Vec<Vec<String>> {
    let mut new_options: Vec<Vec<String>> = Vec::new();
    for option in option {
        let last_node = option.last().unwrap();
        let last_node = find_node(graph, last_node).unwrap();
        for neighbor in &graph[last_node].neighbors {
            if !option.contains(&neighbor.0) {
                let mut new_option = option.clone();
                new_option.push(neighbor.0.clone());
                new_options.push(new_option);
            }
        }
    }
    new_options
}

fn calculate_neighbor_distance(pos: usize, match_name: &String, graph: &[Node]) -> isize {
    for neighbor in &graph[pos].neighbors {
        if neighbor.0 == *match_name {
            return neighbor.1;
        }
    }
    0
}

fn calculate_arrangement(arrangement: &[String], graph: &[Node]) -> isize {
    let mut distance = 0;
    for n in arrangement.windows(2) {
        let node_pos_1 = find_node(graph, &n[0]).unwrap();
        let node_pos_2 = find_node(graph, &n[1]).unwrap();
        distance += calculate_neighbor_distance(node_pos_1, &n[1], graph);
        distance += calculate_neighbor_distance(node_pos_2, &n[0], graph);
    }
    let n_1 = &arrangement[0];
    let n_2 = &arrangement.last().unwrap();
    let node_pos_1 = find_node(graph, n_1).unwrap();
    let node_pos_2 = find_node(graph, n_2).unwrap();
    distance += calculate_neighbor_distance(node_pos_1, n_2, graph);
    distance += calculate_neighbor_distance(node_pos_2, n_1, graph);
    distance
}

fn main() {
    // Load input
    let input = load_input();
    let mut graph: Vec<Node> = Vec::new();
    for line in input {
        let line = line.replace('.', "");
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let node_name_1 = line[0].to_string();
        let node_name_2 = line[10].to_string();
        let distance = match *line.get(2).unwrap() {
            "gain" => line[3].parse::<isize>().unwrap(),
            "lose" => -line[3].parse::<isize>().unwrap(),
            _ => panic!("Unknown input"),
        };
        add_edge(&mut graph, &node_name_1, &node_name_2, distance);
    }
    for i in 0..graph.len() {
        let node_name = graph[i].name.clone();
        add_edge(&mut graph, &node_name, "Me", 0);
        add_edge(&mut graph, "Me", &node_name, 0);
    }
    let mut options: Vec<Vec<String>> = vec![vec![graph[0].name.clone()]];
    for _ in 0..graph.len() - 1 {
        options = calculate_options(options, &graph);
    }
    let res = options
        .iter()
        .map(|x| calculate_arrangement(x, &graph))
        .max()
        .unwrap();
    println!("{res:#?}");
}
