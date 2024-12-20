use helpers::load_input;

mod helpers;

const DANDELING_STACKS: usize = 1;
const CYCLE_STACKS: usize = 4;
const OFFSET_STACKS: usize = 1;
const COMMAND_INDEX_OFFSET: usize = 1;

fn calculate_stacks_amounts(len: usize) -> usize {
    (len - DANDELING_STACKS) / CYCLE_STACKS + OFFSET_STACKS
}

fn calculate_stack_index(len: usize) -> usize {
    calculate_stacks_amounts(len) - 1
}

fn parse_crates(stacks_input: &[String]) -> Vec<Vec<char>> {
    let input_size = stacks_input[0].len();
    let mut stacks = vec![Vec::new(); calculate_stacks_amounts(input_size)];
    for row in stacks_input.iter().rev() {
        let parced_row = row.chars().collect::<Vec<char>>();
        let mut parced_row_index = OFFSET_STACKS;
        while parced_row_index < parced_row.len() {
            let stacks_index = calculate_stack_index(parced_row_index);
            if parced_row[parced_row_index] != ' ' {
                stacks[stacks_index].push(parced_row[parced_row_index]);
            }
            parced_row_index += CYCLE_STACKS;
        }
    }
    stacks
}

fn parse_command(command: &str) -> (usize, usize, usize) {
    // move 5 from 4 to 5
    let command = command.split(' ').collect::<Vec<&str>>();
    let a = command[1].parse::<usize>().unwrap();
    let b = command[3].parse::<usize>().unwrap();
    let c = command[5].parse::<usize>().unwrap();
    (a, b, c)
}

fn execute_command(command: (usize, usize, usize), stacks: &mut [Vec<char>]) {
    let mut moved_crates_index = 0;
    let mut moved_crates = Vec::new();
    while moved_crates_index < command.0 {
        let cr = stacks[command.1 - COMMAND_INDEX_OFFSET].pop().unwrap();
        moved_crates.push(cr);
        moved_crates_index += 1;
    }
    for cr in moved_crates.iter().rev() {
        stacks[command.2 - COMMAND_INDEX_OFFSET].push(*cr);
    }
}

fn print_answer(stacks: &[Vec<char>]) {
    for stack in stacks {
        stack.last().map(|ch| {
            print!("{ch}");
            ch
        });
    }
    println!(" ");
}

fn main() {
    let input = load_input();
    let mut separator = 0;
    for line in &input {
        if line.is_empty() {
            break;
        }
        separator += 1;
    }
    let mut stacks = parse_crates(&input[0..separator - 1]);
    let commands = &input[separator + 1..];

    for command in commands {
        let parced_command = parse_command(command);
        execute_command(parced_command, &mut stacks);
    }

    print_answer(&stacks);
}
