use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .collect::<Vec<Vec<i128>>>();

    let mut result = 0;
    for line in lines {
        result += line.last().unwrap().clone();
        let mut new_line = line;
        loop {
            new_line = new_line
                .windows(2)
                .map(|a| a[1] - a[0])
                .collect::<Vec<i128>>();
            result += new_line.last().unwrap();
            if new_line.iter().all(|values| values == &0) {
                break;
            }
        }
    }
    println!("{}", result);
}
