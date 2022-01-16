use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops;

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Copy, Clone)]
enum Axis {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

type Permutation = (Axis, Axis, Axis);

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, point: Point) -> Point {
        Point {
            x: self.x - point.x,
            y: self.y - point.y,
            z: self.z - point.z,
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, point: Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        &self.x == &other.x && &self.y == &other.y && &self.z == &other.z
    }
}

impl Point {
    fn abs(self: Point) -> Point {
        Point {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn distance(self, point: Point) -> f32 {
        let Point { x, y, z } = self - point;
        ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt()
    }

    fn manhattan_distance(self, point: Point) -> f32 {
        let Point { x, y, z } = (self - point).abs();
        (x + y + z) as f32
    }

    fn permutate(self, permutation: Permutation) -> Point {
        let change = |axis: Axis| match axis {
            Axis::PosX => self.x,
            Axis::NegX => -self.x,
            Axis::PosY => self.y,
            Axis::NegY => -self.y,
            Axis::PosZ => self.z,
            Axis::NegZ => -self.z,
        };

        Point {
            x: change(permutation.0),
            y: change(permutation.1),
            z: change(permutation.2),
        }
    }
}

const ALL_PERMUTATIONS: &[Permutation] = &[
    (Axis::PosX, Axis::PosY, Axis::PosZ),
    (Axis::PosX, Axis::NegY, Axis::NegZ),
    (Axis::PosX, Axis::PosZ, Axis::NegY),
    (Axis::PosX, Axis::NegZ, Axis::PosY),
    (Axis::NegX, Axis::PosY, Axis::NegZ),
    (Axis::NegX, Axis::NegY, Axis::PosZ),
    (Axis::NegX, Axis::PosZ, Axis::PosY),
    (Axis::NegX, Axis::NegZ, Axis::NegY),
    (Axis::PosY, Axis::PosX, Axis::NegZ),
    (Axis::PosY, Axis::NegX, Axis::PosZ),
    (Axis::PosY, Axis::PosZ, Axis::PosX),
    (Axis::PosY, Axis::NegZ, Axis::NegX),
    (Axis::NegY, Axis::PosX, Axis::PosZ),
    (Axis::NegY, Axis::NegX, Axis::NegZ),
    (Axis::NegY, Axis::PosZ, Axis::NegX),
    (Axis::NegY, Axis::NegZ, Axis::PosX),
    (Axis::PosZ, Axis::PosX, Axis::PosY),
    (Axis::PosZ, Axis::NegX, Axis::NegY),
    (Axis::PosZ, Axis::PosY, Axis::NegX),
    (Axis::PosZ, Axis::NegY, Axis::PosX),
    (Axis::NegZ, Axis::PosY, Axis::PosX),
    (Axis::NegZ, Axis::NegY, Axis::NegX),
    (Axis::NegZ, Axis::PosX, Axis::NegY),
    (Axis::NegZ, Axis::NegX, Axis::PosY),
];

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let scanners_beacons: Vec<Vec<Point>> = read_scanners_beacons(reader);

    let mut scanner_to_position: HashMap<usize, Point> = HashMap::new();
    scanner_to_position.insert(0, Point { x: 0, y: 0, z: 0 });

    let mut scanner_to_permutation: HashMap<usize, (usize, Permutation)> = HashMap::new();
    let mut not_found_scanners: Vec<usize> = (1..scanners_beacons.len()).collect();
    let mut scanners_queue: VecDeque<usize> = VecDeque::with_capacity(scanners_beacons.len());
    scanners_queue.push_back(0);
    while let Some(i) = scanners_queue.pop_front() {
        for j in not_found_scanners.clone() {
            let equivalent_points =
                scanners_overlap_heuristic(&scanners_beacons[i], &scanners_beacons[j]);
            if equivalent_points.is_none() {
                continue;
            }

            let (sa_pair, mut sb_pair) = equivalent_points.unwrap();
            let permutation_one = permutation(&sa_pair, &sb_pair);
            let permutation_two = permutation(&sa_pair, &(sb_pair.1, sb_pair.0));
            let mut permutation = permutation_one;
            if permutation_one.is_none() && permutation_two.is_none() {
                continue;
            }

            if permutation_one.is_none() {
                permutation = permutation_two;
                sb_pair = (sb_pair.1, sb_pair.0);
            }

            let permutation = permutation.unwrap();
            scanner_to_permutation.insert(j, (i, permutation));

            let difference = sa_pair.0 - sb_pair.0.permutate(permutation);
            if scanners_overlap(
                &scanners_beacons[i],
                &scanners_beacons[j],
                difference,
                permutation,
            ) {
                let difference = scanner_position(
                    &j,
                    &sa_pair.0,
                    &sb_pair.0,
                    &scanner_to_permutation,
                    &scanner_to_position,
                );
                scanner_to_position.insert(j as usize, difference);
                scanners_queue.push_back(j);
                not_found_scanners.retain(|&x| x != j);
            }
        }
    }

    let mut max_manhattan_distance = 0;
    for (i, a) in scanner_to_position.iter() {
        for (j, b) in scanner_to_position.iter() {
            println!(
                "{}, {}, {:?}, {:?}, {}",
                i,
                j,
                a,
                b,
                a.manhattan_distance(*b)
            );
            max_manhattan_distance =
                cmp::max(max_manhattan_distance, a.manhattan_distance(*b) as u64);
        }
    }

    println!("{}", max_manhattan_distance);
}

fn read_scanners_beacons(reader: BufReader<std::fs::File>) -> Vec<Vec<Point>> {
    let mut scanners_beacons: Vec<Vec<Point>> = vec![];
    let mut scanner_beacons: Vec<Point> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            scanners_beacons.push(scanner_beacons);
            scanner_beacons = vec![];
        } else if &line[..3] != "---" {
            let numbers = line
                .split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            scanner_beacons.push(Point {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            });
        }
    }
    scanners_beacons.push(scanner_beacons);

    scanners_beacons
}

fn scanners_overlap_heuristic(
    sa_points: &[Point],
    sb_points: &[Point],
) -> Option<((Point, Point), (Point, Point))> {
    let mut sa_pair_points = (sa_points[0], sa_points[0]);
    let mut sb_pair_points = (sa_points[0], sa_points[0]);

    let positions = |n| (0..n).cartesian_product(0..n).filter(|(i, j)| i != j);

    let mut overlapping_beacons = 0;
    for (i, j) in positions(sa_points.len()) {
        let d1 = sa_points[i].distance(sa_points[j]);
        for (a, b) in positions(sb_points.len()) {
            let d2 = sb_points[a].distance(sb_points[b]);
            if d1 == d2 {
                overlapping_beacons += 1;
                if overlapping_beacons == 1 {
                    sa_pair_points = (sa_points[i], sa_points[j]);
                    sb_pair_points = (sb_points[a], sb_points[b]);
                }
                break;
            }
        }
    }

    if overlapping_beacons >= 12 * 11 {
        return Some((sa_pair_points, sb_pair_points));
    }

    None
}

fn permutation(
    sa_points_pair: &(Point, Point),
    sb_points_pair: &(Point, Point),
) -> Option<Permutation> {
    let sa_diff = sa_points_pair.0 - sa_points_pair.1;
    let sb_diff = sb_points_pair.0 - sb_points_pair.1;

    for &permutation in ALL_PERMUTATIONS {
        if sa_diff == sb_diff.permutate(permutation) {
            return Some(permutation);
        }
    }

    None
}

fn scanners_overlap(
    sa_beacons: &[Point],
    sb_beacons: &[Point],
    difference: Point,
    permutation: Permutation,
) -> bool {
    let overlapping_beacons = sb_beacons
        .iter()
        .map(|beacon| beacon.permutate(permutation) + difference)
        .fold(0, |total, beacon| {
            total + sa_beacons.contains(&beacon) as u8
        });

    overlapping_beacons >= 12
}

fn scanner_position(
    scanner_index: &usize,
    point1: &Point,
    point2: &Point,
    scanner_to_permutation: &HashMap<usize, (usize, Permutation)>,
    scanner_to_position: &HashMap<usize, Point>,
) -> Point {
    let mut n = &0;
    let mut k = scanner_index;
    let mut scanner_position = *point1;
    while let Some((i, permutation)) = scanner_to_permutation.get(k) {
        if n == &0 {
            scanner_position = *point1 - point2.permutate(*permutation);
            n = i;
        } else {
            scanner_position = scanner_position.permutate(*permutation);
        }
        k = i;
    }
    scanner_position + *scanner_to_position.get(n).unwrap()
}
