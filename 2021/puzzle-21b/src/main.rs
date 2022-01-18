use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let roll_frequency: HashMap<u8, u8> = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
        .iter()
        .cloned()
        .collect();

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let positions = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|x| String::from(x))
                .collect::<Vec<String>>()
        })
        .map(|pieces| pieces[4].parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut game_states = HashMap::new();
    game_states.insert((0, 0, positions[0], positions[1]), 1u128);
    let mut turn = 0;
    let mut victories = (0, 0);
    while game_states.len() > 0 {
        let mut new_game_states: HashMap<(u8, u8, u8, u8), u128> = HashMap::new();
        for ((p1_score, p2_score, p1_position, p2_position), universes) in game_states.clone() {
            if p1_score >= 21 {
                victories.0 += universes;
                continue;
            } else if p2_score >= 21 {
                victories.1 += universes;
                continue;
            }
            for (&increase, &frequency) in &roll_frequency {
                let entry = if turn == 0 {
                    let new_position = mod_range(1, 10, p1_position + increase);
                    let new_score = p1_score + new_position;
                    (new_score, p2_score, new_position, p2_position)
                } else {
                    let new_position = mod_range(1, 10, p2_position + increase);
                    let new_score = p2_score + new_position;
                    (p1_score, new_score, p1_position, new_position)
                };
                *new_game_states.entry(entry).or_insert(0) += universes * frequency as u128;
            }
        }
        game_states = new_game_states;
        turn = (turn + 1) % 2;
    }

    println!("{}", std::cmp::max(victories.0, victories.1));
}

fn mod_range(start: u8, end: u8, value: u8) -> u8 {
    ((value - start) % end) + start
}
