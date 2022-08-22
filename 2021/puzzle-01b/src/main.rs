use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let values: Vec<u16> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();

    let mut number_of_increases = 0;
    let window_slice = 3;
    for i in 0..values.len() - window_slice {
        let current_slice_value: u16 = values[i..i + window_slice].iter().sum();
        let next_slice_value: u16 = values[i + 1..i + window_slice + 1].iter().sum();
        if next_slice_value > current_slice_value {
            number_of_increases += 1;
        }
    }

    print!("{}", number_of_increases);
}
