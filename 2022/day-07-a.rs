use helpers::load_input;

mod helpers;

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> File {
        File { size }
    }
}

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    size: usize,
    parent: Option<usize>,
    files: Vec<File>,
    childrens: Vec<usize>,
}

impl Dir<'_> {
    fn new(name: &str, parent: Option<usize>) -> Dir<'_> {
        Dir {
            name,
            size: 0,
            parent,
            files: Vec::new(),
            childrens: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Disk<'a>(Vec<Dir<'a>>);

fn calculate_dir_size(disk: &mut Disk, dir_position: usize) {
    let file_size = disk.0[dir_position].files.iter().fold(0, |mut sum, file| {
        sum += file.size;
        sum
    });
    for target_children in 0..disk.0[dir_position].childrens.len() {
        let children = disk.0[dir_position].childrens[target_children];
        calculate_dir_size(disk, children);
    }
    let childrens_size = disk.0[dir_position]
        .childrens
        .iter()
        .fold(0, |mut sum, dir_position| {
            sum += disk.0[*dir_position].size;
            sum
        });
    disk.0[dir_position].size = file_size + childrens_size;
}

fn main() {
    // Load input
    let input = load_input();
    let home = Dir::new("/", None);
    let mut disk = Disk(vec![home]);
    let mut current_dir = 0;
    for line in &input[1..] {
        let line = line.split(' ').collect::<Vec<&str>>();
        if line[0] == "$" {
            if line[1] == "cd" {
                match *line.get(2).unwrap() {
                    ".." => {
                        current_dir = disk.0[current_dir].parent.unwrap();
                    }
                    dir_name => {
                        current_dir = *disk.0[current_dir]
                            .childrens
                            .iter()
                            .find(|children_pos| {
                                if disk.0[**children_pos].name == dir_name {
                                    return true;
                                }
                                false
                            })
                            .unwrap();
                    }
                }
            }
        } else if line[0] == "dir" {
            let children = Dir::new(line[1], Some(current_dir));
            let children_position = disk.0.len();
            disk.0.push(children);
            disk.0[current_dir].childrens.push(children_position);
        } else {
            let file = File::new(line[0].parse::<usize>().unwrap());
            disk.0[current_dir].files.push(file);
        }
    }

    calculate_dir_size(&mut disk, 0);

    let res = disk
        .0
        .iter()
        .filter(|dir| dir.size < 100_000)
        .fold(0, |mut sum, file| {
            sum += file.size;
            sum
        });

    println!("{res}");
}
