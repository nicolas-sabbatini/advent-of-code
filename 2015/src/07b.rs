use std::collections::HashMap;

use input_loader::load_input;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Operation {
    And(String, String),
    AndAsing(u16, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Not(String),
    Assign(u16),
    AssignWire(String),
}

impl Operation {
    fn calculate(
        &self,
        wires: &HashMap<String, Operation>,
        solved_wires: &mut HashMap<Operation, u16>,
    ) -> u16 {
        if solved_wires.contains_key(self) {
            return solved_wires[self];
        }
        let res = match self {
            Operation::And(a, b) => {
                wires[a].calculate(wires, solved_wires) & wires[b].calculate(wires, solved_wires)
            }
            Operation::AndAsing(a, b) => a & wires[b].calculate(wires, solved_wires),
            Operation::Or(a, b) => {
                wires[a].calculate(wires, solved_wires) | wires[b].calculate(wires, solved_wires)
            }
            Operation::LShift(a, b) => wires[a].calculate(wires, solved_wires) << b,
            Operation::RShift(a, b) => wires[a].calculate(wires, solved_wires) >> b,
            Operation::Not(a) => !wires[a].calculate(wires, solved_wires),
            Operation::Assign(a) => *a,
            Operation::AssignWire(a) => wires[a].calculate(wires, solved_wires),
        };
        solved_wires.insert(self.clone(), res);
        res
    }
}

fn main() {
    // Load input
    let input = load_input();
    let mut wires: HashMap<String, Operation> = HashMap::new();
    for line in input {
        let parce_line = line.split_whitespace().collect::<Vec<&str>>();
        let operation = match parce_line.len() {
            3 => match parce_line[0].parse::<u16>() {
                Ok(a) => Operation::Assign(a),
                Err(_) => Operation::AssignWire(parce_line[0].to_string()),
            },
            4 => Operation::Not(parce_line[1].to_string()),
            5 => match parce_line[1] {
                "AND" => match parce_line[0].parse::<u16>() {
                    Ok(a) => Operation::AndAsing(a, parce_line[2].to_string()),
                    Err(_) => Operation::And(parce_line[0].to_string(), parce_line[2].to_string()),
                },
                "OR" => Operation::Or(parce_line[0].to_string(), parce_line[2].to_string()),
                "LSHIFT" => Operation::LShift(
                    parce_line[0].to_string(),
                    parce_line[2].parse::<u16>().unwrap(),
                ),
                "RSHIFT" => Operation::RShift(
                    parce_line[0].to_string(),
                    parce_line[2].parse::<u16>().unwrap(),
                ),
                _ => panic!("Unknown operation"),
            },
            _ => panic!("Unknown operation"),
        };
        let last = parce_line.last().unwrap();
        wires.insert((*last).to_string(), operation);
    }
    wires.insert("b".to_string(), Operation::Assign(3176));
    let mut solved_wires: HashMap<Operation, u16> = HashMap::new();
    let res = wires.get("a").unwrap().calculate(&wires, &mut solved_wires);
    println!("{res}");
}
