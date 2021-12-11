use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut number_of_increases = -1;
    let mut previous_value: u16 = 0;
    for line in reader.lines() {
        let current_value = line.unwrap().parse::<u16>().unwrap();
        if current_value > previous_value {
            number_of_increases += 1;
        }
        previous_value = current_value;
    }

    print!("{}", number_of_increases);
}
