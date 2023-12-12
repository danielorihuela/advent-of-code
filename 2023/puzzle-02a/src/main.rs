use std::fs;

use regex::Regex;

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"([0-9]+) (green|red|blue)").unwrap();
    let mut result = 0;
    'lines: for (i, line) in input.lines().enumerate() {
        for groups in line.split(";") {
            for (_, [number, color]) in re.captures_iter(groups).map(|c| c.extract()) {
                let value = number.parse::<u8>().unwrap();
                if color == "red" && value > MAX_RED {
                    continue 'lines;
                } else if color == "green" && value > MAX_GREEN {
                    continue 'lines;
                } else if color == "blue" && value > MAX_BLUE {
                    continue 'lines;
                }
            }
        }
        result += i + 1;
    }

    println!("{}", result);
}