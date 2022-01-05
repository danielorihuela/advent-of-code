use std::fs::File;
use std::io::{prelude::*, BufReader};

use pathfinding::dijkstra;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let risks: Vec<Vec<u32>> = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let result = dijkstra(
        &(0, 0),
        |(x, y)| {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|(x_increment, y_increment)| {
                    risks
                        .get((x + x_increment) as usize)
                        .and_then(|row_risks| row_risks.get((y + y_increment) as usize))
                        .map(|&risk| ((x + x_increment, y + y_increment), risk))
                })
                .filter(|x| !x.is_none())
                .map(|x| x.unwrap())
                .collect::<Vec<_>>()
        },
        |&p| p == (risks.len() as i32 - 1, risks[0].len() as i32 - 1),
    );

    println!("{:?}", result.unwrap().1);
}
