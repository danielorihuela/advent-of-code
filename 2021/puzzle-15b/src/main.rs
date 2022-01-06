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

    let num_rows = risks.len() as i32;
    let num_columns = risks[0].len() as i32;
    let result = dijkstra(
        &(0, 0),
        |(x, y)| {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|(x_increment, y_increment)| (x + x_increment, y + y_increment))
                .filter(|(x, y)| x < &(num_columns * 5) && y < &(num_rows * 5))
                .map(|(x, y)| {
                    risks
                        .get((y % num_rows) as usize)
                        .and_then(|row_risks| row_risks.get((x % num_columns) as usize))
                        .map(|&risk| {
                            let mut new_risk = risk as i32 + (x / num_columns) + (y / num_rows);
                            if new_risk > 9 {
                                new_risk -= 9;
                            }
                            ((x, y), new_risk)
                        })
                })
                .filter(|p| !p.is_none())
                .map(|p| p.unwrap())
                .collect::<Vec<_>>()
        },
        |&p| p == (num_rows * 5 - 1, num_columns * 5 - 1),
    );

    println!("{}", result.unwrap().1);
}
