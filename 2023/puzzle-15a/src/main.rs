use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result: usize = input.split(",").map(hash).sum();
    println!("{}", result);
}

fn hash(value: &str) -> usize {
    value
        .as_bytes()
        .iter()
        .fold(0, |acc, &value| (17 * (acc + value as usize)) % 256)
}
