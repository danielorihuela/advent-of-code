use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Range = (i128, i128);
type Cuboid = (Range, Range, Range);

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let current_cuboid_status = status(&lines[0]);
    let current_cuboid = cuboid(&lines[0]);
    let mut cuboids: Vec<(Cuboid, u8)> = vec![];
    cuboids.push((current_cuboid, current_cuboid_status));
    for line in lines {
        let current_cuboid_status = status(&line);
        let current_cuboid = cuboid(&line);

        let new_cuboids: Vec<(Cuboid, u8)> = cuboids
            .iter()
            .map(|&(cuboid, status)| (intersection(current_cuboid, cuboid), (status + 1) % 2))
            .filter(|(intersection, _)| intersection.is_some())
            .map(|(intersection, status)| (intersection.unwrap(), status))
            .collect();

        cuboids = [cuboids, new_cuboids].concat();
        if current_cuboid_status == 1 {
            cuboids = [cuboids, vec![(current_cuboid, current_cuboid_status)]].concat();
        }
    }

    let total = cuboids.iter().fold(0, |total, &(cuboid, status)| {
        total
            + if status == 1 {
                cuboid_size(cuboid)
            } else {
                -cuboid_size(cuboid)
            }
    });
    println!("{}", total);
}

fn status(line: &str) -> u8 {
    (&line[..2] == "on") as u8
}

fn cuboid(line: &str) -> (Range, Range, Range) {
    let pieces: Vec<&str> = line
        .split(|c| c == '=' || c == '.' || c == ',')
        .map(|c| c)
        .collect();

    let digit = |piece: &str| piece.parse::<i128>().unwrap();
    (
        (digit(pieces[1]), digit(pieces[3]) + 1),
        (digit(pieces[5]), digit(pieces[7]) + 1),
        (digit(pieces[9]), digit(pieces[11]) + 1),
    )
}

fn cuboid_size(cuboid: Cuboid) -> i128 {
    let ((x0, x1), (y0, y1), (z0, z1)) = cuboid;

    (x1 - x0) * (y1 - y0) * (z1 - z0)
}

fn intersection(cuboid1: Cuboid, cuboid2: Cuboid) -> Option<Cuboid> {
    let (cuboid1_x, cuboid1_y, cuboid1_z) = cuboid1;
    let (cuboid2_x, cuboid2_y, cuboid2_z) = cuboid2;

    let x_intersection = axis_intersection(cuboid1_x, cuboid2_x);
    let y_intersection = axis_intersection(cuboid1_y, cuboid2_y);
    let z_intersection = axis_intersection(cuboid1_z, cuboid2_z);

    if x_intersection.is_none() || y_intersection.is_none() || z_intersection.is_none() {
        return None;
    }

    Some((
        x_intersection.unwrap(),
        y_intersection.unwrap(),
        z_intersection.unwrap(),
    ))
}

fn axis_intersection(range1: Range, range2: Range) -> Option<Range> {
    if (range1.0 <= range2.0 && range2.0 <= range1.1)
        || (range2.0 <= range1.0 && range1.0 <= range2.1)
    {
        return Some((cmp::max(range1.0, range2.0), cmp::min(range1.1, range2.1)));
    }

    None
}
