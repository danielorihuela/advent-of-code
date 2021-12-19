use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let dots = read_dots(&mut reader);
    let splits = read_splits(&mut reader);
    let width = max_x(&dots) + 1;
    let height = max_y(&dots) + 1;

    let mut grid = build_grid(width, height);
    for (x, y) in dots {
        grid[y][x] = '#';
    }

    let mut new_grid = grid.clone();
    for (direction, distance) in splits {
        if direction == 'y' {
            new_grid = fold_y(&new_grid, distance);
        } else if direction == 'x' {
            new_grid = fold_x(&new_grid, distance);
        }
        // For the first part we only require to perform the first split
        break;
    }

    println!("{}", count_visible_dots(&new_grid));
}

fn read_dots(reader: &mut BufReader<File>) -> Vec<(usize, usize)> {
    let mut dots: Vec<(usize, usize)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            break;
        }

        let position = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        dots.push((position[0], position[1]));
    }

    dots
}

fn read_splits(reader: &mut BufReader<File>) -> Vec<(char, usize)> {
    let mut splits: Vec<(char, usize)> = vec![];
    for line in reader.lines() {
        let pieces = line
            .unwrap()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let split = &pieces[2].split("=").collect::<Vec<&str>>();
        let direction = split[0].chars().collect::<Vec<char>>()[0];
        let distance = split[1].parse::<usize>().unwrap();
        splits.push((direction, distance));
    }

    splits
}

fn max_x(dots: &[(usize, usize)]) -> usize {
    let dots_x = dots.iter().map(|(x, _)| x);

    *dots_x.max().unwrap()
}

fn max_y(dots: &[(usize, usize)]) -> usize {
    let dots_y = dots.iter().map(|(_, y)| y);

    *dots_y.max().unwrap()
}

fn build_grid(width: usize, height: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; width]; height]
}

fn fold_y(grid: &Vec<Vec<char>>, distance: usize) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid.clone();
    let mut z = distance;
    for i in distance..new_grid.len() {
        for j in 0..new_grid[0].len() {
            if new_grid[i][j] == '#' {
                new_grid[z][j] = '#';
            }
        }
        z = z.wrapping_sub(1);
    }

    new_grid[0..distance].to_vec()
}

fn fold_x(grid: &Vec<Vec<char>>, distance: usize) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid.clone();
    for i in 0..new_grid.len() {
        let mut z = distance;
        for j in distance..new_grid[i].len() {
            if new_grid[i][j] == '#' {
                new_grid[i][z] = '#';
            }
            z = z.wrapping_sub(1);
        }
    }

    new_grid
        .into_iter()
        .map(|s| s.into_iter().take(distance).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn count_visible_dots(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .flat_map(|row| row.iter().filter(|&dot| *dot == '#'))
        .count()
}
