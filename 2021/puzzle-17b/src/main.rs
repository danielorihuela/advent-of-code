use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let splitted_input: Vec<&str> = input
        .split(|c| c == '=' || c == ',' || c == '.')
        .map(|x| x)
        .collect();

    let (x0, x1, y0, y1) = (
        splitted_input[1].parse::<i64>().unwrap(),
        splitted_input[3].parse::<i64>().unwrap(),
        splitted_input[7].parse::<i64>().unwrap(),
        splitted_input[5].parse::<i64>().unwrap(),
    );

    let mut total = 0;
    for x_velocity in 0..x1 + 1 {
        for y_velocity in y1..y1.abs() {
            let mut i = 0;
            let (mut x, mut y) = (0, 0);
            while x <= x1 && y >= y1 {
                if x0 <= x && x <= x1 && y1 <= y && y <= y0 {
                    total += 1;
                    break;
                }
                if x_velocity - i > 0 {
                    x += x_velocity - i;
                }
                y += y_velocity - i;
                i += 1;
            }
        }
    }
    println!("{}", total);
}
