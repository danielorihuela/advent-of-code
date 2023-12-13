use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result: u16 = input
        .lines()
        .map(|line| line.split([':', '|']).collect::<Vec<&str>>())
        .map(|line| -> (HashSet<u8>, HashSet<u8>) {
            (
                HashSet::from_iter(sorted_numbers(line[1])),
                HashSet::from_iter(sorted_numbers(line[2])),
            )
        })
        .map(|(winning_numbers, my_numbers)| {
            winning_numbers
                .intersection(&my_numbers)
                .into_iter()
                .cloned()
                .collect::<Vec<u8>>()
                .len()
        })
        .filter(|intersection_len| intersection_len > &0)
        .map(|intersection_len| 2u16.pow(intersection_len as u32 - 1))
        .sum();
    println!("{}", result);
}

fn sorted_numbers(values: &str) -> Vec<u8> {
    let mut values = values
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    values.sort();

    values
}
