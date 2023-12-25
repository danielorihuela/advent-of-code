use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut rocks_to_cycle = HashMap::<Vec<Vec<char>>, usize>::new();
    let mut rocks = matrix;
    let mut i = 0usize;
    while i < 1000000000 {
        // tilt north
        rocks = transpose(&rocks);
        rocks = slide_rocks(&rocks);
        rocks = transpose(&rocks);
        // tilt west
        rocks = slide_rocks(&rocks);
        // tilt south
        rocks = turn_upside_down(&rocks);
        rocks = transpose(&rocks);
        rocks = slide_rocks(&rocks);
        rocks = transpose(&rocks);
        rocks = turn_upside_down(&rocks);
        // tilt east
        rocks = turn_left_right(&rocks);
        rocks = slide_rocks(&rocks);
        rocks = turn_left_right(&rocks);

        if let Some(cycle) = rocks_to_cycle.get(&rocks) {
            let diff = cycle.abs_diff(i);
            i += diff * ((1000000000 - i) / diff);
        }

        rocks_to_cycle.insert(rocks.clone(), i);
        i += 1;
    }

    let mut result = 0;
    for mut line in transpose(&rocks) {
        line.reverse();
        result += line
            .iter()
            .enumerate()
            .fold(0, |acc, (i, value)| match value {
                'O' => acc + i + 1,
                _ => acc,
            });
    }

    println!("{}", result);
}

fn transpose(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..matrix.len())
        .map(|row| {
            (0..matrix[row].len())
                .map(|column| matrix[column][row])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn turn_upside_down(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..matrix.len())
        .rev()
        .map(|row| matrix[row].clone())
        .collect::<Vec<Vec<char>>>()
}

fn turn_left_right(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    matrix
        .iter()
        .map(|row| {
            let mut copy = row.clone();
            copy.reverse();

            copy
        })
        .collect::<Vec<Vec<char>>>()
}

fn slide_rocks(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut slided_rocks = vec![];
    for line in matrix {
        let line_string = line.iter().map(|x| x.to_string()).collect::<String>();
        let sections = line_string.split_inclusive("#");
        let bytes = sections.map(|x| x.as_bytes().to_vec());
        let tilted_rocks = bytes
            .flat_map(|mut x| {
                x.sort();
                x.reverse();
                x
            })
            .map(|x| x as char)
            .collect::<Vec<char>>();
        slided_rocks.push(tilted_rocks);
    }

    slided_rocks
}
