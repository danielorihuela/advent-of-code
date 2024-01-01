use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Compress graph
    let mut new_graph = HashMap::new();
    let mut origins = VecDeque::new();
    origins.push_back((1, 0));
    let mut visited = HashSet::new();
    while !origins.is_empty() {
        let origin = origins.pop_front().unwrap();
        if visited.contains(&origin) {
            continue;
        }
        visited.insert(origin);

        let intersections = next_intersections(&matrix, origin);
        let next_nodes = new_graph.entry(origin).or_insert(HashSet::new());
        for intersection in intersections {
            next_nodes.insert(intersection);
            origins.push_back(intersection.0);
        }
    }

    // DFS Compressed graph
    let mut max_path_length = 0;
    let mut queue = VecDeque::new();
    for element in new_graph.get(&(1, 0)).unwrap() {
        queue.push_back((element.0, vec![(1, 0)], element.1));
    }
    while !queue.is_empty() {
        let (position, path, cost) = queue.pop_back().unwrap();
        if path.contains(&position) {
            continue;
        }
        if position == (matrix[0].len() - 2, matrix.len() - 1) {
            max_path_length = max_path_length.max(cost);
            continue;
        }

        let new_path = [path, vec![position]].concat();
        for element in new_graph.get(&position).unwrap() {
            queue.push_back((element.0, new_path.clone(), cost + element.1));
        }
    }
    println!("{}", max_path_length);
}

fn next_intersections(
    matrix: &[Vec<char>],
    origin: (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let mut intersections = vec![];

    let mut queue = VecDeque::new();
    queue.push_back((origin, 0usize));
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let (position, cost) = queue.pop_front().unwrap();
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);

        if position == (matrix[0].len() - 2, matrix.len() - 1) {
            intersections.push((position, cost));
            break;
        }

        let (x, y) = position;
        if position != origin {
            let intersection = {
                let mut paths = 0;
                if x > 0 && matrix[y][x - 1] != '#' {
                    paths += 1;
                }
                if matrix[y][x + 1] != '#' {
                    paths += 1;
                }
                if y > 0 && matrix[y - 1][x] != '#' {
                    paths += 1;
                }
                if matrix[y + 1][x] != '#' {
                    paths += 1;
                }

                paths >= 3
            };
            if intersection {
                intersections.push((position, cost));
                continue;
            }
        }

        if x > 0 && matrix[y][x - 1] != '#' {
            queue.push_back(((x - 1, y), cost + 1));
        }
        if x < matrix[0].len() - 1 && matrix[y][x + 1] != '#' {
            queue.push_back(((x + 1, y), cost + 1));
        }
        if y > 0 && matrix[y - 1][x] != '#' {
            queue.push_back(((x, y - 1), cost + 1));
        }
        if y < matrix.len() - 1 && matrix[y + 1][x] != '#' {
            queue.push_back(((x, y + 1), cost + 1));
        }
    }

    intersections
}
