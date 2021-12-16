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

    let mut risk = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if is_low_point(&matrix, i, j) {
                risk += value + 1;
            }
        }
    }

    println!("{}", risk);
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
