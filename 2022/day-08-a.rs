use helpers::load_input;

mod helpers;

fn is_visible_in_row(tree_position: usize, tree: usize, row: &[usize]) -> bool {
    let mut visible_left = true;
    let mut visible_right = true;
    for neighbour_tree in row.iter().take(tree_position) {
        if *neighbour_tree >= tree {
            visible_left = false;
            break;
        }
    }
    for neighbour_tree in row.iter().skip(tree_position + 1) {
        if *neighbour_tree >= tree {
            visible_right = false;
            break;
        }
    }
    visible_left || visible_right
}

fn is_visible_in_column(
    tree_position_x: usize,
    tree_position_y: usize,
    tree: usize,
    map: &[Vec<usize>],
) -> bool {
    let mut visible_top = true;
    let mut visible_bottom = true;
    for row_of_trees in map.iter().take(tree_position_y) {
        if row_of_trees[tree_position_x] >= tree {
            visible_top = false;
            break;
        }
    }
    for row_of_trees in map.iter().skip(tree_position_y + 1) {
        if row_of_trees[tree_position_x] >= tree {
            visible_bottom = false;
            break;
        }
    }
    visible_top || visible_bottom
}

fn main() {
    // Load input
    let input = load_input();
    let mut map = Vec::new();
    for line in input {
        let mut row = Vec::new();
        for tree in line.chars() {
            row.push(tree.to_string().parse::<usize>().unwrap());
        }
        map.push(row);
    }
    let mut visible_trees = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            if x == 0
                || y == 0
                || x == map.len() - 1
                || y == row.len() - 1
                || is_visible_in_row(x, *tree, row)
                || is_visible_in_column(x, y, *tree, &map)
            {
                visible_trees += 1;
            }
        }
    }
    println!("{visible_trees:?}");
}
