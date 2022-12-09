use helpers::load_input;

mod helpers;

fn visible_score_in_row(tree_position: usize, tree: &usize, row: &[usize]) -> usize {
    let mut visible_left = 0;
    let mut visible_right = 0;
    for neighbour_tree in row.iter().take(tree_position).rev() {
        visible_left += 1;
        if neighbour_tree >= tree {
            break;
        }
    }
    for neighbour_tree in row.iter().skip(tree_position + 1) {
        visible_right += 1;
        if neighbour_tree >= tree {
            break;
        }
    }
    visible_left.max(1) * visible_right.max(1)
}

fn visible_score_in_column(
    tree_position_x: usize,
    tree_position_y: usize,
    tree: &usize,
    map: &[Vec<usize>],
) -> usize {
    let mut visible_top = 0;
    let mut visible_bottom = 0;
    for row_of_trees in map.iter().take(tree_position_y).rev() {
        visible_top += 1;
        if row_of_trees[tree_position_x] >= *tree {
            break;
        }
    }
    for row_of_trees in map.iter().skip(tree_position_y + 1) {
        visible_bottom += 1;
        if row_of_trees[tree_position_x] >= *tree {
            break;
        }
    }
    visible_top.max(1) * visible_bottom.max(1)
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
    let mut best_visible_score = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let tree_visible_score =
                visible_score_in_row(x, tree, row) * visible_score_in_column(x, y, tree, &map);
            best_visible_score = best_visible_score.max(tree_visible_score);
        }
    }

    println!("{best_visible_score:?}");
}
