use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"([0-9]+) (green|red|blue)").unwrap();
    let mut result = 0;
    for line in input.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for (_, [number, color]) in re.captures_iter(line).map(|c| c.extract()) {
            let value = number.parse::<u32>().unwrap();
            if color == "red" && value > max_red {
                max_red = value;
            } else if color == "green" && value > max_green {
                max_green = value;
            } else if color == "blue" && value > max_blue {
                max_blue = value;
            }
        }
        result += max_red * max_green * max_blue;
    }

    println!("{}", result);
}
