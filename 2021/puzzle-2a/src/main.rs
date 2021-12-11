use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut position = (0, 0);
    for line in reader.lines() {
        let command: String = line.unwrap();
        let (x, y) = movement(command);
        position.0 += x;
        position.1 += y;
    }

    print!("{}", i32::from(position.0) * i32::from(position.1));
}

fn movement(command: String) -> (i16, i16) {
    let pieces = command.split_ascii_whitespace().collect::<Vec<&str>>();
    let (direction, movement) = (pieces[0], pieces[1].parse::<i16>().unwrap());

    match direction {
        "forward" => (movement, 0),
        "up" => (0, -movement),
        "down" => (0, movement),
        _ => (0, 0),
    }
}
