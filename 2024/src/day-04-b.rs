use input_loader::load_input;

const XMAS: [char; 3] = ['m', 'a', 's'];

fn find_down_left(x: usize, y: usize, map: &[Vec<char>], char_i: usize) -> bool {
    if y + 1 < map.len() && x > 0 && map[y + 1][x - 1] == XMAS[char_i] {
        return true;
    }
    false
}

fn find_down_right(x: usize, y: usize, map: &[Vec<char>], char_i: usize) -> bool {
    if y + 1 < map.len() && x + 1 < map[0].len() && map[y + 1][x + 1] == XMAS[char_i] {
        return true;
    }
    false
}

fn find_up_left(x: usize, y: usize, map: &[Vec<char>], char_i: usize) -> bool {
    if y > 0 && x > 0 && map[y - 1][x - 1] == XMAS[char_i] {
        return true;
    }
    false
}

fn find_up_right(x: usize, y: usize, map: &[Vec<char>], char_i: usize) -> bool {
    if y > 0 && x + 1 < map[0].len() && map[y - 1][x + 1] == XMAS[char_i] {
        return true;
    }
    false
}

// 0-- 2--
// -1- -1-
// --2 --0
fn find_1x(x: usize, y: usize, map: &[Vec<char>]) -> bool {
    let type1 = find_up_left(x, y, map, 0) && find_down_right(x, y, map, 2);
    let type2 = find_up_left(x, y, map, 2) && find_down_right(x, y, map, 0);
    type1 || type2
}

// --0 --2
// -1- -1-
// 2-- 0--
fn find_2x(x: usize, y: usize, map: &[Vec<char>]) -> bool {
    let type1 = find_up_right(x, y, map, 0) && find_down_left(x, y, map, 2);
    let type2 = find_up_right(x, y, map, 2) && find_down_left(x, y, map, 0);
    type1 || type2
}

fn find_xmas(x: usize, y: usize, map: &[Vec<char>]) -> usize {
    let mut res = 0;
    if map[y][x] == XMAS[1] && find_1x(x, y, map) && find_2x(x, y, map) {
        res += 1;
    }
    res
}

fn main() {
    let input = load_input();
    let mut map = Vec::new();
    for line in input {
        let row = line.to_lowercase().chars().collect::<Vec<_>>();
        map.push(row);
    }
    let mut res = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            res += find_xmas(x, y, &map);
        }
    }
    println!("{res}");
}
