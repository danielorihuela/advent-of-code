use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let races_data = input
        .lines()
        .map(|line| {
            line.split(": ")
                .last()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<u128>>()
        })
        .collect::<Vec<Vec<u128>>>();

    let result = races_data[0]
        .iter()
        .zip(races_data[1].iter())
        .map(|(&time, &record_distance)| {
            let min_time_holding = (1..time)
                .find(|i| i * (time - i) > record_distance)
                .unwrap();

            time - 2 * min_time_holding + 1
        })
        .reduce(|acc, value| acc * value)
        .unwrap();
    println!("{}", result);
}
