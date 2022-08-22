use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut values: Vec<u16> = vec![];
    for line in reader.lines() {
        values = line
            .unwrap()
            .split(",")
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
    }

    values.sort();
    let max = values[values.len() - 1];
    let mut average = f32::MAX;
    let mut perfect_spot = 0;
    for i in 0..max + 1 {
        let new_values: Vec<u32> = values
            .iter()
            .map(|&x| step_cost(x, i))
            .collect::<Vec<u32>>();
        let new_average = new_values.iter().sum::<u32>() as f32 / new_values.len() as f32;
        if new_average > average {
            perfect_spot = i - 1;
            break;
        }

        average = new_average;
    }

    let mut total_cost = 0;
    for value in values {
        total_cost += step_cost(value, perfect_spot) as u32;
    }

    println!("{}", total_cost);
}

fn step_cost(from: u16, to: u16) -> u32 {
    let difference = i32::abs(from as i32 - to as i32);
    ((difference * (difference + 1)) / 2) as u32
}
