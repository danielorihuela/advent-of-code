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

    let (x0, y1) = (
        splitted_input[1].parse::<i64>().unwrap(),
        splitted_input[5].parse::<i64>().unwrap(),
    );

    let mut x_velocity = 0;
    while termial(x_velocity) < x0 {
        x_velocity += 1;
    }

    let y_velocity = y1.abs() - 1;

    println!("{},{}", x_velocity, y_velocity);
    println!("{}", termial(y_velocity));
}

fn termial(n: i64) -> i64 {
    (n * (n + 1)) / 2
}
