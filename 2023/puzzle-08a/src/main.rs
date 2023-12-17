use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input.lines().collect::<Vec<&str>>();
    let instructions = data[0].chars().collect::<Vec<char>>();
    let mut node_to_node = HashMap::new();
    for line in data[2..].iter() {
        let line_data = line.split(['=', ' ', ',', '(', ')']).collect::<Vec<&str>>();
        node_to_node.insert(line_data[0], (line_data[4], line_data[6]));
    }

    let mut step = 0;
    let mut state = "AAA";
    loop {
        let possibilities = node_to_node.get(state).unwrap();
        if instructions[step % instructions.len()] == 'L' {
            state = possibilities.0;
        } else {
            state = possibilities.1;
        }
        step += 1;

        if state == "ZZZ" {
            break;
        }
    }
    println!("{}", step);
}
