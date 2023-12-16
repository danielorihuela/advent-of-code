use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|data| {
            (
                data[0].chars().collect::<Vec<char>>(),
                data[1].parse::<u128>().unwrap(),
            )
        });

    let mut extended_data = data
        .map(|(hand, bid)| (hand, bid.clone()))
        .collect::<Vec<(Vec<char>, u128)>>();

    extended_data.sort_by_key(|item| {
        (
            hand_value(&item.0),
            card_value(&item.0[0]),
            card_value(&item.0[1]),
            card_value(&item.0[2]),
            card_value(&item.0[3]),
            card_value(&item.0[4]),
        )
    });

    let result = extended_data
        .iter()
        .enumerate()
        .fold(0, |acc, (i, data)| acc + (i + 1) as u128 * data.1);
    println!("{}", result);
}

fn hand_value(hand: &[char]) -> u128 {
    let mut counts = HashMap::new();

    hand.iter().for_each(|character| {
        *counts.entry(character).or_insert(0) += 1u128;
    });
    let num_jokers = counts.remove(&'J');
    if let Some(5) = num_jokers {
        return 7;
    }

    let mut values = counts.values().cloned().collect::<Vec<u128>>();
    values.sort_by(|a, b| b.cmp(a));
    if let Some(num_jokers) = num_jokers {
        values[0] += num_jokers;
    }

    match &values[..] {
        &[5] => 7,
        &[4, 1] => 6,
        &[3, 2] => 5,
        &[3, 1, 1] => 4,
        &[2, 2, 1] => 3,
        &[2, 1, 1, 1] => 2,
        &[1, 1, 1, 1, 1] => 1,
        _ => 0,
    }
}

fn card_value(character: &char) -> u128 {
    match character {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    }
}
