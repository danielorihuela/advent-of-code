use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let total = input
        .lines()
        .map(|line| {
            let first_number = line.chars().find(|c| c.is_numeric()).unwrap();
            let last_number = line.chars().rev().find(|c| c.is_numeric()).unwrap();

            format!("{}{}", first_number, last_number)
                .parse::<u16>()
                .unwrap()
        })
        .sum::<u16>();

    println!("{}", total);
}
