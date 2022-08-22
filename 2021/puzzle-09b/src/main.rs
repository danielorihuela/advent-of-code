use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u16>> = vec![];
    for line in reader.lines() {
        let row = line
            .unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u16)
            .collect::<Vec<u16>>();
        matrix.push(row);
    }

    let mut basin_sizes: Vec<u32> = vec![];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if is_low_point(&matrix, i, j) {
                basin_sizes.push(calculate_basin_size(&matrix, i, j));
            }
        }
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn is_low_point(matrix: &[Vec<u16>], row: usize, column: usize) -> bool {
    let actual_value = matrix[row][column];
    let mut is_low_point = true;
    if row + 1 < matrix.len() {
        is_low_point = is_low_point && actual_value < matrix[row + 1][column];
    }
    if column + 1 < matrix[0].len() {
        is_low_point = is_low_point && actual_value < matrix[row][column + 1];
    }

    if let Some(value) = row.checked_sub(1) {
        is_low_point = is_low_point && actual_value < matrix[value][column];
    }
    if let Some(value) = column.checked_sub(1) {
        is_low_point = is_low_point && actual_value < matrix[row][value];
    }

    is_low_point
}

fn calculate_basin_size(matrix: &[Vec<u16>], row: usize, column: usize) -> u32 {
    let mut visited: Vec<(usize, usize)> = vec![];
    dfs_visit(matrix, row, column, &mut visited);

    visited.len() as u32
}

fn dfs_visit(matrix: &[Vec<u16>], row: usize, column: usize, visited: &mut Vec<(usize, usize)>) {
    let mut position_already_visited = false;
    for (visited_row, visited_column) in visited.clone() {
        if row == visited_row && column == visited_column {
            position_already_visited = true;
        }
    }
    if position_already_visited {
        return;
    }

    visited.push((row, column));

    if let Some(value) = neighbour_value(matrix, row as i8 + 1, column as i8) {
        if value != 9 {
            dfs_visit(matrix, row + 1, column, visited);
        }
    }
    if let Some(value) = neighbour_value(matrix, row as i8 - 1, column as i8) {
        if value != 9 {
            dfs_visit(matrix, row - 1, column, visited);
        }
    }
    if let Some(value) = neighbour_value(matrix, row as i8, column as i8 + 1) {
        if value != 9 {
            dfs_visit(matrix, row, column + 1, visited);
        }
    }
    if let Some(value) = neighbour_value(matrix, row as i8, column as i8 - 1) {
        if value != 9 {
            dfs_visit(matrix, row, column - 1, visited);
        }
    }
}

fn neighbour_value(matrix: &[Vec<u16>], row: i8, column: i8) -> Option<u16> {
    if row > matrix.len() as i8 - 1 || row < 0 || column > matrix[0].len() as i8 - 1 || column < 0 {
        return None;
    }

    Some(matrix[row as usize][column as usize])
}
