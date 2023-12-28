use std::{
    collections::{HashSet, VecDeque},
    fs,
};
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|tokens| (tokens[0], tokens[1].parse::<usize>().unwrap()))
        .collect::<Vec<(&str, usize)>>();
    let mut edges = vec![];
    let mut position = (0i32, 0i32);
    for (direction, steps) in data {
        let increment = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "D" => (0, 1),
            "U" => (0, -1),
            _ => (0, 0),
        };
        for _ in 0..steps {
            position = (position.0 + increment.0, position.1 + increment.1);
            edges.push(position);
        }
    }
    let max_x = *edges.iter().map(|(x, _)| x).max().unwrap();
    let min_x = *edges.iter().map(|(x, _)| x).min().unwrap();
    let max_y = *edges.iter().map(|(_, y)| y).max().unwrap();
    let min_y = *edges.iter().map(|(_, y)| y).min().unwrap();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((1, 1)); // start with position inside lagoon
    while !queue.is_empty() {
        let position = queue.pop_front().unwrap();
        if visited.contains(&position) || edges.contains(&position) {
            continue;
        }

        visited.insert(position);
        let (x, y) = position;
        if x > min_x {
            queue.push_back((x - 1, y));
        }
        if x < max_x {
            queue.push_back((x + 1, y));
        }
        if y > min_y {
            queue.push_back((x, y - 1));
        }
        if y < max_y {
            queue.push_back((x, y + 1));
        }
    }
    println!("{}", edges.len() + visited.len());
}
