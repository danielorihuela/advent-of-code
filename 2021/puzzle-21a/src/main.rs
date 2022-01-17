use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
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
        .map(|pieces| pieces[4].parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut current_dice = 0;
    let mut player_one = (0, positions[0]);
    let mut player_two = (0, positions[1]);
    let mut turn = 0;
    while player_one.0 < 1000 && player_two.0 < 1000 {
        let roll = (1..4)
            .into_iter()
            .fold(0, |total, next| total + current_dice + next);
        current_dice += 3;

        if turn % 2 == 0 {
            player_one.1 = mod_range(1, 10, player_one.1 + roll);
            player_one.0 += player_one.1
        } else {
            player_two.1 = mod_range(1, 10, player_two.1 + roll);
            player_two.0 += player_two.1
        }
        turn += 3;
    }

    if player_one.0 < 1000 {
        println!("{}", turn * player_one.0);
    } else {
        println!("{}", turn * player_two.0);
    }
}

fn mod_range(start: u64, end: u64, value: u64) -> u64 {
    ((value - start) % end) + start
}
