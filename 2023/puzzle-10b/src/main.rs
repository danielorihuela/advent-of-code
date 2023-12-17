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
    let start_row = matrix
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(&'S'))
        .unwrap()
        .0;
    let start_column = matrix[start_row]
        .iter()
        .enumerate()
        .find(|(_, &character)| character == 'S')
        .unwrap()
        .0;

    let mut visited = HashMap::new();
    visited.insert((start_row, start_column), 0);
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_row, start_column));
    while !to_visit.is_empty() {
        let (y, x) = to_visit.pop_front().unwrap();

        let next_positions = next_positions(&matrix, (y as usize, x as usize));
        for position in next_positions {
            if visited.get(&position).is_some() {
                continue;
            }

            visited.insert(position, visited.get(&(y, x)).unwrap() + 1);
            to_visit.push_back(position);
        }
    }

    // part 2
    let mut result = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if visited.get(&(i, j)).is_some() {
                continue;
            }

            let mut possible_intersections = vec![];
            for k in j + 1..matrix[0].len() {
                if visited.get(&(i, k)).is_none() {
                    continue;
                }

                let c = matrix[i][k];
                if c == 'J' || c == 'L' || c == 'F' || c == '7' || c == '|' || c == 'S' {
                    possible_intersections.push(c);
                }
            }
            let mut intersections = 0;
            for k in 0..possible_intersections.len() {
                if possible_intersections[k] == '|' {
                    intersections += 1;
                } else if possible_intersections[k] == 'L'
                    && possible_intersections.get(k + 1) == Some(&&'7')
                {
                    intersections += 1;
                } else if possible_intersections[k] == 'F'
                    && possible_intersections.get(k + 1) == Some(&&'J')
                {
                    intersections += 1;
                } else if possible_intersections[k] == 'S'
                    && possible_intersections.get(k + 1) == Some(&&'J')
                {
                    intersections += 1;
                } else if possible_intersections[k] == 'S'
                    && possible_intersections.get(k + 1) == Some(&&'7')
                {
                    intersections += 1;
                } else if possible_intersections[k] == 'L'
                    && possible_intersections.get(k + 1) == Some(&&'S')
                {
                    intersections += 1;
                } else if possible_intersections[k] == 'F'
                    && possible_intersections.get(k + 1) == Some(&&'S')
                {
                    intersections += 1;
                }
            }

            if intersections % 2 != 0 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}

fn next_positions(matrix: &[Vec<char>], (y, x): (usize, usize)) -> Vec<(usize, usize)> {
    let mut next_posible_postions = vec![];
    if can_go_right(&matrix, (y, x)) {
        next_posible_postions.push((y, x + 1));
    }
    if can_go_left(&matrix, (y, x)) {
        next_posible_postions.push((y, x - 1));
    }
    if can_go_up(&matrix, (y, x)) {
        next_posible_postions.push((y - 1, x));
    }
    if can_go_down(&matrix, (y, x)) {
        next_posible_postions.push((y + 1, x));
    }

    next_posible_postions
}

fn can_go_right(matrix: &[Vec<char>], (y, x): (usize, usize)) -> bool {
    if x == matrix[0].len() - 1 {
        return false;
    }

    let state_a = matrix[y][x];
    let state_b = matrix[y][x + 1];
    (state_a == 'S' || state_a == '-' || state_a == 'L' || state_a == 'F')
        && (state_b == '-' || state_b == 'J' || state_b == '7')
}

fn can_go_left(matrix: &[Vec<char>], (y, x): (usize, usize)) -> bool {
    if x == 0 {
        return false;
    }

    let state_a = matrix[y][x];
    let state_b = matrix[y][x - 1];
    (state_a == 'S' || state_a == '-' || state_a == 'J' || state_a == '7')
        && (state_b == '-' || state_b == 'F' || state_b == 'L')
}

fn can_go_up(matrix: &[Vec<char>], (y, x): (usize, usize)) -> bool {
    if y == 0 {
        return false;
    }

    let state_a = matrix[y][x];
    let state_b = matrix[y - 1][x];
    (state_a == 'S' || state_a == '|' || state_a == 'J' || state_a == 'L')
        && (state_b == '|' || state_b == '7' || state_b == 'F')
}

fn can_go_down(matrix: &[Vec<char>], (y, x): (usize, usize)) -> bool {
    if y == matrix.len() - 1 {
        return false;
    }

    let state_a = matrix[y][x];
    let state_b = matrix[y + 1][x];
    (state_a == 'S' || state_a == '|' || state_a == '7' || state_a == 'F')
        && (state_b == '|' || state_b == 'J' || state_b == 'L')
}
