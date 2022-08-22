use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points: HashMap<(u16, u16), u16> = HashMap::new();

    for line in reader.lines() {
        let ((mut x1, mut y1), (mut x2, mut y2)) = from_to_points(&line.unwrap());
        if x1 != x2 && y1 != y2 {
            continue;
        }

        if x1 == x2 {
            swap_if_greater(&mut y1, &mut y2);
            for i in y1..y2 + 1 {
                *points.entry((x1, i)).or_insert(0) += 1u16;
            }
        } else if y1 == y2 {
            swap_if_greater(&mut x1, &mut x2);
            for i in x1..x2 + 1 {
                *points.entry((i, y1)).or_insert(0) += 1u16;
            }
        }
    }

    let mut sum = 0;
    for (_, &value) in points.iter() {
        if value > 1 {
            sum += 1;
        }
    }

    print!("{}", sum);
}

fn from_to_points(line: &str) -> ((u16, u16), (u16, u16)) {
    let points_splits = line
        .split_whitespace()
        .filter(|&splits| splits != "->")
        .flat_map(|point| {
            point
                .split(",")
                .map(|axis_point| axis_point.parse::<u16>().unwrap())
        })
        .collect::<Vec<u16>>();

    (
        (points_splits[0], points_splits[1]),
        (points_splits[2], points_splits[3]),
    )
}

fn swap_if_greater(a: &mut u16, b: &mut u16) {
    if *a > *b {
        std::mem::swap(a, b);
    }
}