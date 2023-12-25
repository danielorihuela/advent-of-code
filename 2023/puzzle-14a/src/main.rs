use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let matrix_t = (0..matrix.len())
        .map(|row| {
            (0..matrix[row].len())
                .map(|column| matrix[column][row].to_string())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let mut result = 0;
    for line in matrix_t {
        let sections = line.split_inclusive("#");
        let bytes = sections.map(|x| x.as_bytes().to_vec());
        let tilted_rocks = bytes
            .flat_map(|mut x| {
                x.sort();
                x.reverse();
                x
            })
            .rev();
        result += tilted_rocks
            .enumerate()
            .fold(0, |acc, (i, value)| match value {
                b'O' => acc + i + 1,
                _ => acc,
            });
    }
    println!("{}", result);
}
