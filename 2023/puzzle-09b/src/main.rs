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
        let mut partial_result = line.first().unwrap().clone();
        let mut new_line = line;
        let mut i = 0;
        loop {
            new_line = new_line
                .windows(2)
                .map(|a| a[1] - a[0])
                .collect::<Vec<i128>>();
            partial_result = if i == 0 {
                i = 1;
                partial_result - new_line.first().unwrap()
            } else {
                i = 0;
                partial_result + new_line.first().unwrap()
            };
            if new_line.iter().all(|values| values == &0) {
                break;
            }
        }
        result += partial_result;
    }
    println!("{}", result);
}
