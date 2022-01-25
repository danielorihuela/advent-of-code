use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let value = |line: &str| {
        line.split_whitespace()
            .nth(2)
            .unwrap()
            .parse::<i128>()
            .unwrap()
    };
    let inputs = (0..lines.len())
        .step_by(18)
        .map(|i| (value(&lines[i + 5]), value(&lines[i + 15])))
        .collect::<Vec<(i128, i128)>>();

    for i in (1111111..9999999).rev() {
        let digits: Vec<i128> = i
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i128)
            .collect();

        let mut z = 0;
        let mut position = 0;
        let mut current_number = [0i128; 14];
        for (i, &(increase, increase2)) in inputs.iter().enumerate() {
            if increase > 9 {
                z = 26 * z + digits[position] + increase2;
                current_number[i] = digits[position];
                position += 1;
            } else {
                current_number[i] = (z % 26) + increase;
                z = z / 26;
            }
            if current_number[i] < 1 || current_number[i] > 9 {
                z = 1;
                break;
            }
        }

        if z == 0 {
            println!(
                "{}",
                current_number
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<String>()
            );
            break;
        }
    }
}
