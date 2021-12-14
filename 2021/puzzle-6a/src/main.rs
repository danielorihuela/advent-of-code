use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lanternfishes: Vec<u8> = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split(",")
                .map(|value| value.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for _ in 1..80 {
        let mut new_lanternfishes: Vec<u8> = vec![];
        for lanternfish in lanternfishes.iter_mut() {
            *lanternfish -= 1;
            if *lanternfish == 0 {
                *lanternfish = 7;
                new_lanternfishes.push(9);
            }
        }
        lanternfishes.extend(new_lanternfishes);
    }
    println!("{}", lanternfishes.len());
}
