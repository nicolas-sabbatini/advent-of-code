use input_loader::load_input;

const XMAS: [char; 4] = ['x', 'm', 'a', 's'];

fn find_left(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if x > 0 && map[y][x - 1] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_left(x - 1, y, map, char_i + 1);
    }
    false
}

fn find_right(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if x + 1 < map[0].len() && map[y][x + 1] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_right(x + 1, y, map, char_i + 1);
    }
    false
}

fn find_down(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if y + 1 < map.len() && map[y + 1][x] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_down(x, y + 1, map, char_i + 1);
    }
    false
}

fn find_up(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if y > 0 && map[y - 1][x] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_up(x, y - 1, map, char_i + 1);
    }
    false
}

fn find_down_left(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if y + 1 < map.len() && x > 0 && map[y + 1][x - 1] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_down_left(x - 1, y + 1, map, char_i + 1);
    }
    false
}

fn find_down_right(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if y + 1 < map.len() && x + 1 < map[0].len() && map[y + 1][x + 1] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_down_right(x + 1, y + 1, map, char_i + 1);
    }
    false
}

fn find_up_left(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if y > 0 && x > 0 && map[y - 1][x - 1] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_up_left(x - 1, y - 1, map, char_i + 1);
    }
    false
}

fn find_up_right(x: usize, y: usize, map: &Vec<Vec<char>>, char_i: usize) -> bool {
    if y > 0 && x + 1 < map[0].len() && map[y - 1][x + 1] == XMAS[char_i] {
        if char_i == 3 {
            return true;
        }
        return find_up_right(x + 1, y - 1, map, char_i + 1);
    }
    false
}

fn find_xmas(x: usize, y: usize, map: &Vec<Vec<char>>) -> usize {
    let mut res = 0;
    if map[y][x] == XMAS[0] {
        res += usize::from(find_down(x, y, map, 1))
            + usize::from(find_up(x, y, map, 1))
            + usize::from(find_right(x, y, map, 1))
            + usize::from(find_left(x, y, map, 1))
            + usize::from(find_down_right(x, y, map, 1))
            + usize::from(find_down_left(x, y, map, 1))
            + usize::from(find_up_right(x, y, map, 1))
            + usize::from(find_up_left(x, y, map, 1));
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
