use std::{collections::HashMap, fs, mem::swap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input.lines().collect::<Vec<&str>>();
    let instructions = data[0].chars().collect::<Vec<char>>();

    let mut node_to_node = HashMap::new();
    let mut states = vec![];
    for line in data[2..].iter() {
        let line_data = line.split(['=', ' ', ',', '(', ')']).collect::<Vec<&str>>();
        node_to_node.insert(line_data[0], (line_data[4], line_data[6]));
        if line_data[0].ends_with('A') {
            states.push(line_data[0]);
        }
    }

    let mut step = 0;
    let mut steps = states.iter().map(|_| 0).collect::<Vec<usize>>();
    loop {
        states = states
            .iter()
            .map(|state| {
                let possibilities = node_to_node.get(state).unwrap();
                if instructions[step % instructions.len()] == 'L' {
                    possibilities.0
                } else {
                    possibilities.1
                }
            })
            .collect::<Vec<&str>>();

        step += 1;

        states.iter().enumerate().for_each(|(i, state)| {
            if state.ends_with('Z') {
                steps[i] = step as usize;
            }
        });

        if steps.iter().all(|step| step != &0) {
            break;
        }
    }
    println!(
        "{}",
        steps
            .into_iter()
            .reduce(|acc, value| lcm(acc, value))
            .unwrap()
    );
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(mut max: usize, mut min: usize) -> usize {
    if min > max {
        swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
