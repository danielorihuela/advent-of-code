use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut result = 0;
    let re = Regex::new(r"([0-9]+)").unwrap();
    for (y, line) in input.lines().enumerate() {
        for (x, number) in re
            .find_iter(line)
            .map(|c| (c.range().next().unwrap(), c.as_str()))
        {
            let surrounding_positions = surrounding_positions(&matrix, (x, y), number.len());
            if next_to_symbol(&matrix, &surrounding_positions) {
                result += number.parse::<u32>().unwrap();
            }
        }
    }
    println!("{}", result);
}

fn surrounding_positions(
    matrix: &[Vec<char>],
    (x, y): (usize, usize),
    number_length: usize,
) -> Vec<(usize, usize)> {
    let mut surrounding_positions = vec![];
    let x0 = if x > 0 { x - 1 } else { 0 };
    let xf = if x + number_length >= matrix[0].len() {
        matrix[0].len() - 1
    } else {
        x + number_length
    };
    for i in x0..xf + 1 {
        if y > 0 {
            surrounding_positions.push((i, y - 1));
        }
        if i == x0 || i == xf {
            surrounding_positions.push((i, y));
        }
        if y < matrix.len() - 1 {
            surrounding_positions.push((i, y + 1));
        }
    }

    surrounding_positions
}

fn next_to_symbol(matrix: &[Vec<char>], surrounding_positions: &[(usize, usize)]) -> bool {
    surrounding_positions
        .iter()
        .any(|(x, y)| matrix[*y][*x] != '.' && !matrix[*y][*x].is_ascii_digit())
}
