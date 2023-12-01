use helpers::load_input;

mod helpers;

#[derive(Clone, Debug)]
struct Node {
    f_score: usize,
    g_score: usize,
    prev_node: Option<(usize, usize)>,
}

fn main() {
    // Load input
    let input = load_input();
    println!("{input:?}");
    // Parce inputs
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in &input {
        map.push(Vec::new());
        let row = map.len() - 1;
        for num in line.chars() {
            let num = num.to_string().parse::<usize>().unwrap();
            map[row].push(num);
        }
    }

    let target = (map[0].len() - 1, map.len() - 1);

    let mut nodes = vec![
        vec![
            Node {
                f_score: usize::MAX,
                g_score: usize::MAX,
                prev_node: None,
            };
            target.0
        ];
        target.1
    ];

    nodes[0][0] = Node {
        f_score: 0,
        g_score: 0,
        prev_node: None,
    };

    let mut open_set: Vec<(usize, usize)> = vec![(0, 0)];
    let mut close_set: Vec<(usize, usize)> = Vec::new();

    loop {
        break;
    }

    for row in map.iter() {
        println!("{:?}", row);
    }

    println!("{:?}", target);
}
