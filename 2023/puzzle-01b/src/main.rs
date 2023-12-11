use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let total = input
        .lines()
        .map(|line| {
            let first_number = number_as_digit(&find_number(line, &numbers));
            let last_number = number_as_digit(&rfind_number(line, &numbers));

            format!("{}{}", first_number, last_number)
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>();

    println!("{}", total);
}

fn find_number(line: &str, values: &[&str]) -> String {
    let min = values
        .iter()
        .map(|value| (value, line.find(value)))
        .filter(|(_, pos)| pos.is_some())
        .min_by_key(|(_, pos)| *pos);

    min.unwrap().0.to_string()
}

fn rfind_number(line: &str, values: &[&str]) -> String {
    let max = values
        .iter()
        .map(|value| (value, line.rfind(value)))
        .filter(|(_, pos)| pos.is_some())
        .max_by_key(|(_, pos)| *pos);

    max.unwrap().0.to_string()
}

fn number_as_digit(number: &str) -> u64 {
    match number {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}
