use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let races_data = input
        .lines()
        .map(|line| {
            line.split(": ")
                .last()
                .unwrap()
                .replace(" ", "")
                .parse::<u128>()
                .unwrap()
        })
        .collect::<Vec<u128>>();

    let time: u128 = races_data[0];
    let record_distance = races_data[1];
    let min_time_holding = (1..time)
        .find(|i| i * (time - i) > record_distance)
        .unwrap();

    let result = time - 2 * min_time_holding + 1;
    println!("{}", result);
}
