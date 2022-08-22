use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut values: Vec<i32> = vec![];
    for line in reader.lines() {
         values = line.unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    }

    values.sort();
    let median_value = values[(values.len() / 2) - 1];
    let mut total_cost = 0;
    for value in values {
        total_cost += i32::abs(value - median_value);
    }

    println!("{}", total_cost);
}
