use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut origin = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'S' {
                origin = (j, i);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((origin, 0usize));
    while !queue.is_empty() {
        let ((x, y), step) = queue.pop_front().unwrap();
        if visited.contains_key(&(x, y)) {
            continue;
        }
        visited.insert((x, y), step);

        if x > 0 {
            if matrix[y][x - 1] != '#' {
                queue.push_back(((x - 1, y), step + 1));
            }
        }
        if x < matrix.len() - 1 {
            if matrix[y][x + 1] != '#' {
                queue.push_back(((x + 1, y), step + 1));
            }
        }
        if y > 0 {
            if matrix[y - 1][x] != '#' {
                queue.push_back(((x, y - 1), step + 1));
            }
        }
        if y < matrix[0].len() - 1 {
            if matrix[y + 1][x] != '#' {
                queue.push_back(((x, y + 1), step + 1));
            }
        }
    }

    let mut total = 0;
    let step = 64;
    for i in (step % 2..step + 1).into_iter().step_by(2) {
        total += visited.iter().filter(|(_, &step)| step == i).count();
    }
    println!("{}", total);
}
