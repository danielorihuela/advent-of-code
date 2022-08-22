use std::collections::BTreeMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut days_to_fishes: BTreeMap<u8, u64> = BTreeMap::new();
    for i in 0..9 {
        days_to_fishes.insert(i, 0);
    }

    let increase =
        |map: &mut BTreeMap<u8, u64>, key: u8, value: u64| *map.entry(key).or_insert(0) += value;
    for line in reader.lines() {
        for split in line.unwrap().split(",") {
            let key: u8 = split.parse::<u8>().unwrap();
            increase(&mut days_to_fishes, key, 1);
        }
    }

    let update =
        |map: &mut BTreeMap<u8, u64>, key: u8, value: u64| *map.entry(key).or_insert(0) = value;
    for _ in 1..256 {
        for i in 0..days_to_fishes.len() - 1 {
            let new_fishes = *days_to_fishes.get(&(i as u8 + 1)).unwrap_or(&0);
            update(&mut days_to_fishes, i as u8, new_fishes);
        }
        let fishes_at_zero = *days_to_fishes.get(&0).unwrap();
        increase(&mut days_to_fishes, 7, fishes_at_zero);
        update(&mut days_to_fishes, 9, fishes_at_zero);
        update(&mut days_to_fishes, 0, 0);
    }

    println!("{}", days_to_fishes.values().sum::<u64>());
}
