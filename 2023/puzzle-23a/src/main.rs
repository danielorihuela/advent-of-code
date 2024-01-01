use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut max_path_length = 0;
    let origin = (1, 0);
    let mut queue = VecDeque::new();
    queue.push_back((origin, vec![], 0usize));
    while !queue.is_empty() {
        let (position, path, cost) = queue.pop_back().unwrap();
        if path.contains(&position) {
            continue;
        }
        if position == (matrix[0].len() - 2, matrix.len() - 1) {
            max_path_length = max_path_length.max(cost);
        }

        let (x, y) = position;
        let new_path = [path, vec![(x, y)]].concat();
        match matrix[y][x] {
            '>' => {
                queue.push_back(((x + 1, y), new_path, cost + 1));
                continue;
            }
            '<' => {
                queue.push_back(((x - 1, y), new_path, cost + 1));
                continue;
            }
            'v' => {
                queue.push_back(((x, y + 1), new_path, cost + 1));
                continue;
            }
            '^' => {
                queue.push_back(((x, y - 1), new_path, cost + 1));
                continue;
            }
            _ => (),
        }

        if x > 0 && matrix[y][x - 1] != '#' {
            queue.push_back(((x - 1, y), new_path.clone(), cost + 1));
        }
        if x < matrix[0].len() - 1 && matrix[y][x + 1] != '#' {
            queue.push_back(((x + 1, y), new_path.clone(), cost + 1));
        }
        if y > 0 && matrix[y - 1][x] != '#' {
            queue.push_back(((x, y - 1), new_path.clone(), cost + 1));
        }
        if y < matrix.len() - 1 && matrix[y + 1][x] != '#' {
            queue.push_back(((x, y + 1), new_path, cost + 1));
        }
    }
    println!("{}", max_path_length);
}
