use std::fs::File;
use std::io::{prelude::*, BufReader};

type Range = (i32, i32);

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut space = vec![vec![vec![0u8; 101]; 101]; 101];

    for line in reader.lines() {
        let line = line.unwrap();
        let status = status(&line);
        let ranges = ranges(&line);
        if outside_area(ranges) {
            continue;
        }
        space = turn_cuboids(space.clone(), ranges, status);
    }

    println!("{}", on_cuboids(space));
}

fn status(line: &str) -> u8 {
    (&line[..2] == "on") as u8
}

fn ranges(line: &str) -> (Range, Range, Range) {
    let pieces: Vec<&str> = line
        .split(|c| c == '=' || c == '.' || c == ',')
        .map(|c| c)
        .collect();

    let digit = |piece: &str| piece.parse::<i32>().unwrap();
    (
        (digit(pieces[1]) + 50, digit(pieces[3]) + 51),
        (digit(pieces[5]) + 50, digit(pieces[7]) + 51),
        (digit(pieces[9]) + 50, digit(pieces[11]) + 51),
    )
}

fn outside_area(ranges: (Range, Range, Range)) -> bool {
    let ((x0, x1), (y0, y1), (z0, z1)) = ranges;
    0 > x0
        || x0 > 101
        || 0 > x1
        || x1 > 101
        || 0 > y0
        || y0 > 101
        || 0 > y1
        || y1 > 101
        || 0 > z0
        || z0 > 101
        || 0 > z1
        || z1 > 101
}

fn turn_cuboids(
    space: Vec<Vec<Vec<u8>>>,
    ranges: (Range, Range, Range),
    status: u8,
) -> Vec<Vec<Vec<u8>>> {
    let ((x0, x1), (y0, y1), (z0, z1)) = ranges;

    let mut new_space = space.clone();
    for x in x0..x1 {
        for y in y0..y1 {
            for z in z0..z1 {
                new_space[x as usize][y as usize][z as usize] = status;
            }
        }
    }

    new_space
}

fn on_cuboids(space: Vec<Vec<Vec<u8>>>) -> u32 {
    let mut total = 0;
    for x in 0..space.len() {
        for y in 0..space[0].len() {
            for z in 0..space[0][0].len() {
                if space[x][y][z] == 1 {
                    total += 1;
                }
            }
        }
    }

    total
}
