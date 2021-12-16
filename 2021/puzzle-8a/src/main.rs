use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Quantity of numbers 1, 4, 7 and 8
    let mut count = 0;
    for line in reader.lines() {
        let data = line
            .unwrap()
            .split("|")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let output = data[1].clone();
        let digit_segments = output
            .split_whitespace()
            .map(|x| x.chars().count())
            .collect::<Vec<usize>>();
        for segments in digit_segments {
            if segments == 2 || segments == 4 || segments == 3 || segments == 7 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
