use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // expand columns
    for j in (0..matrix[0].len()).rev() {
        let mut all_dots = true;
        for i in 0..matrix.len() {
            if matrix[i][j] != '.' {
                all_dots = false;
            }
        }

        if !all_dots {
            continue;
        }
        for i in 0..matrix.len() {
            matrix[i].insert(j, '.');
        }
    }

    // expand rows
    for i in (0..matrix.len()).rev() {
        if matrix[i].iter().any(|c| c != &'.') {
            continue;
        }
        matrix.insert(i, matrix[i].clone());
    }

    // get galaxies
    let mut galaxies = vec![];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // bfs for every pair of galaxies
    let mut result = 0;
    for i in 0..galaxies.len() - 1 {
        let mut visited = HashSet::new();
        let mut visited_galaxies = 0;

        let mut to_visit = VecDeque::new();
        to_visit.push_back((galaxies[i], 0));

        while visited_galaxies != galaxies[i + 1..].len() {
            let ((x, y), parent_value) = to_visit.pop_front().unwrap();
            if visited.get(&(x, y)).is_some() {
                continue;
            }

            let value = parent_value + 1;
            if x < matrix.len() - 1 {
                to_visit.push_back(((x + 1, y), value));
            }
            if y < matrix[0].len() - 1 {
                to_visit.push_back(((x, y + 1), value));
            }
            if x > 0 {
                to_visit.push_back(((x - 1, y), value));
            }
            if y > 0 {
                to_visit.push_back(((x, y - 1), value));
            }

            visited.insert((x, y));

            if galaxies[i + 1..].contains(&(x, y)) {
                visited_galaxies += 1;
                result += value - 1;
            }
        }
    }
    println!("{}", result);
}
