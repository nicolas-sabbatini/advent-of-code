#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    // Parce input
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut map_vis: Vec<Vec<bool>> = Vec::new();
    for line in &input {
        map.push(Vec::new());
        map_vis.push(Vec::new());
        let pos = map.len() - 1;
        for height in line.chars() {
            let height = height.to_string().parse::<u8>().unwrap();
            map[pos].push(height);
            map_vis[pos].push(false);
        }
    }
    // Calculate low points
    let mut low_points: Vec<(isize, isize)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if ((x > 0 && h < &map[y][x - 1]) || x == 0)
                && ((x < row.len() - 1 && h < &map[y][x + 1]) || x == row.len() - 1)
                && ((y > 0 && h < &map[y - 1][x]) || y == 0)
                && ((y < map.len() - 1 && h < &map[y + 1][x]) || y == map.len() - 1)
            {
                low_points.push((x as isize, y as isize));
            }
        }
    }
    // calculate sizes of basins
    let mut basins_sizes: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| calculate_basins(*x, *y, &map, &mut map_vis))
        .collect();
    // Order sizes
    basins_sizes.sort_by(|a, b| b.cmp(a));
    // Calculate res
    let mut res = 1;
    for basin in &basins_sizes[0..3] {
        res *= basin;
    }
    println!("{res}");
}

fn calculate_basins(x: isize, y: isize, map: &Vec<Vec<u8>>, map_vis: &mut Vec<Vec<bool>>) -> usize {
    if x >= 0
        && y >= 0
        && (y as usize) < map.len()
        && (x as usize) < map[y as usize].len()
        && map[y as usize][x as usize] < 9
        && !map_vis[y as usize][x as usize]
    {
        map_vis[y as usize][x as usize] = true;
        return 1
            + calculate_basins(x - 1, y, map, map_vis)
            + calculate_basins(x + 1, y, map, map_vis)
            + calculate_basins(x, y - 1, map, map_vis)
            + calculate_basins(x, y + 1, map, map_vis);
    }
    0
}
