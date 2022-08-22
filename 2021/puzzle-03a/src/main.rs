use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut sums: Vec<u32> = bits_from_string(first_line);

    let mut number_of_lines: u32 = 0;
    for line in reader.lines() {
        let current_values: Vec<u32> = bits_from_string(line.unwrap());
        sums = sums
            .iter()
            .enumerate()
            .map(|(i, &x)| x + current_values[i])
            .collect();
        number_of_lines += 1;
    }

    let gamma_rate: String = most_common_bits(&sums, number_of_lines, false);
    let gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();

    let epsilon_rate: String = most_common_bits(&sums, number_of_lines, true);
    let epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("{}", gamma_rate * epsilon_rate);
}

fn bits_from_string(line: String) -> Vec<u32> {
    line.trim()
        .chars()
        .map(|bit| bit.to_digit(2).unwrap())
        .collect()
}

fn most_common_bits(sums: &[u32], column_length: u32, inverse: bool) -> String {
    {
        sums.iter()
            .map(|&bit_sum| {
                char::from_digit(((bit_sum > column_length / 2) ^ inverse) as u32, 10).unwrap()
            })
            .collect()
    }
}
