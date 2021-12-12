use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let bits_strings: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut oxygen_rating: Vec<String> = bits_strings.clone();
    let mut i = 0;
    while oxygen_rating.len() != 1 {
        let most_common_bit: char = most_common_bit_at_column(i, &oxygen_rating, false);
        oxygen_rating =
            filter_bits_strings_by_most_common_at_column(i, oxygen_rating, most_common_bit);
        i += 1;
    }

    let mut co2_rating: Vec<String> = bits_strings.clone();
    let mut i = 0;
    while co2_rating.len() != 1 {
        let most_common_bit: char = most_common_bit_at_column(i, &co2_rating, true);
        co2_rating = filter_bits_strings_by_most_common_at_column(i, co2_rating, most_common_bit);
        i += 1;
    }

    println!(
        "{}",
        isize::from_str_radix(&oxygen_rating[0], 2).unwrap()
            * isize::from_str_radix(&co2_rating[0], 2).unwrap()
    );
}

fn most_common_bit_at_column(column: usize, bits: &[String], inverse: bool) -> char {
    {
        let sum_bits = bits.iter().fold(0, |total, bits| {
            total + bits_from_string(column, bits).unwrap()
        });
        let number_of_lines = bits.len();
        char::from_digit(
            ((sum_bits >= ((number_of_lines as f64) / 2f64).ceil() as u32) ^ inverse) as u32,
            10,
        )
        .unwrap()
    }
}

fn bits_from_string(column: usize, line: &str) -> Option<u32> {
    Some(line.trim().chars().nth(column)?.to_digit(10)?)
}

fn filter_bits_strings_by_most_common_at_column(
    column: usize,
    bits_strings: Vec<String>,
    most_common_bit: char,
) -> Vec<String> {
    bits_strings
        .into_iter()
        .filter(|bits| bits.chars().nth(column).unwrap() == most_common_bit)
        .collect()
}
