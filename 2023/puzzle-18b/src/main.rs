use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|tokens| tokens[2])
        .map(|token| token.replace("(", "").replace(")", ""))
        .map(|token| to_instruction(token.as_str()))
        .collect::<Vec<((i128, i128), i128)>>();

    let mut edges = vec![];
    let mut position = (0i128, 0i128);
    let mut border_count = 0;
    for ((xi, yi), steps) in data {
        let (x, y) = position;
        position = (x + xi * steps, y + yi * steps);
        border_count += steps;
        edges.push(position);
    }

    let mut area = 0;
    for points in edges.windows(2) {
        let ((x1, y1), (x2, y2)) = (points[0], points[1]);
        area += (y1 + y2) * (x1 - x2);
    }
    area /= 2;

    println!("{}", border_count + area - (border_count / 2) + 1);
}

fn to_instruction(data: &str) -> ((i128, i128), i128) {
    let steps = i128::from_str_radix(&data[1..data.len() - 1], 16).unwrap();
    let direction = match data.chars().last().unwrap() {
        '0' => (1, 0),
        '1' => (0, 1),
        '2' => (-1, 0),
        '3' => (0, -1),
        _ => (0, 0),
    };

    (direction, steps)
}
