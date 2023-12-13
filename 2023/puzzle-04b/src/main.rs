use std::{collections::HashMap, collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut cards_to_quantity = HashMap::<usize, usize>::new();
    for (i, _) in input.lines().enumerate() {
        cards_to_quantity.insert(i, 1);
    }
    for (i, line) in input.lines().enumerate() {
        let card = line.split([':', '|']).collect::<Vec<&str>>();
        let winning_numbers = HashSet::<u8>::from_iter(sorted_numbers(card[1]));
        let my_numbers = HashSet::<u8>::from_iter(sorted_numbers(card[2]));
        let intersection_length = winning_numbers
            .intersection(&my_numbers)
            .into_iter()
            .cloned()
            .collect::<Vec<u8>>()
            .len();

        for j in 0..intersection_length {
            if let Some(value) = cards_to_quantity.get(&(i + 1 + j)) {
                cards_to_quantity.insert(i + 1 + j, value + cards_to_quantity.get(&i).unwrap());
            }
        }
    }

    // println!("{:#?}", cards_to_quantity);
    println!("{}", cards_to_quantity.values().sum::<usize>());
}

fn sorted_numbers(values: &str) -> Vec<u8> {
    let mut values = values
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    values.sort();

    values
}
