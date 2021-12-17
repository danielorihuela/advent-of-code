use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut current_step = 1;
    let mut all_octopuses_flash_first_step = -1;
    while all_octopuses_flash_first_step == -1 {
        let height = matrix.len();
        let width = matrix[0].len();
        for i in 0..height {
            for j in 0..width {
                matrix[i][j] += 1;
                if matrix[i][j] == 10 {
                    flash(&mut matrix, i, j);
                }
            }
        }

        for i in 0..height {
            for j in 0..width {
                if matrix[i][j] > 9 {
                    matrix[i][j] = 0;
                }
            }
        }

        let mut total_sum = 0u16;
        for i in 0..height {
            for j in 0..width {
                total_sum += matrix[i][j] as u16;
            }
        }
        if total_sum == 0 {
            all_octopuses_flash_first_step = current_step;
        }
        current_step += 1;
    }

    println!("{}", all_octopuses_flash_first_step);
}

fn flash(matrix: &mut Vec<Vec<u8>>, row: usize, column: usize) {
    let row = row as i8;
    let column = column as i8;
    let indexes = [
        (row - 1, column - 1),
        (row - 1, column),
        (row - 1, column + 1),
        (row, column + 1),
        (row + 1, column + 1),
        (row + 1, column),
        (row + 1, column - 1),
        (row, column - 1),
    ];

    for (row, column) in indexes {
        if position_inside_grid(matrix, row, column) && matrix[row as usize][column as usize] < 10 {
            matrix[row as usize][column as usize] += 1;
            if matrix[row as usize][column as usize] == 10 {
                flash(matrix, row as usize, column as usize);
            }
        }
    }
}

fn position_inside_grid(matrix: &Vec<Vec<u8>>, row: i8, column: i8) -> bool {
    if row < 0 || row > matrix.len() as i8 - 1 || column < 0 || column > matrix[0].len() as i8 - 1 {
        return false;
    }

    true
}
