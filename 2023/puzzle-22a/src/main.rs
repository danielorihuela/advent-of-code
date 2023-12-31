use std::{collections::HashSet, fs};

type Point = (i128, i128, i128);

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let bars = input
        .lines()
        .map(|line| line.split_once('~').unwrap())
        .map(|points| (str_to_point(points.0), str_to_point(points.1)))
        .collect::<Vec<(Point, Point)>>();

    let mut sorted_bars = bars.clone();
    sorted_bars.sort_by(|a, b| a.1 .2.cmp(&b.1 .2));

    let mut fallen_bars = vec![];
    fallen_bars.push(move_bar_down(sorted_bars[0], 1));
    for &bar in sorted_bars.iter().skip(1) {
        let mut z = bar.0 .2;
        while z > 0 {
            z -= 1;
            if z == 0 {
                fallen_bars.push(move_bar_down(bar, z + 1));
                break;
            }
            let potential_new_bar = move_bar_down(bar, z);
            if fallen_bars
                .iter()
                .filter(|bar| bar.1 .2 == z)
                .any(|&bar| bar_crash(bar, potential_new_bar))
            {
                fallen_bars.push(move_bar_down(bar, z + 1));
                break;
            }
        }
    }

    let mut removable_bars = HashSet::new();
    for &bar in fallen_bars.iter() {
        let z = bar.1 .2;
        let mut removable = true;
        for &upper_bar in fallen_bars.iter().filter(|bar| bar.0 .2 == z + 1) {
            let mut crashes = fallen_bars
                .iter()
                .filter(|bar| bar.1 .2 == z)
                .filter(|&&bar| bar_crash_2d(bar, upper_bar) == true);
            if crashes.clone().count() == 1 && crashes.next().unwrap() == &bar {
                removable = false;
                break;
            }
        }
        if removable {
            removable_bars.insert(bar);
        }
    }
    println!("{:?}", removable_bars.len());
}

fn str_to_point(data: &str) -> Point {
    let point = data
        .split(",")
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    (point[0], point[1], point[2])
}

fn move_bar_down(mut bar: (Point, Point), z: i128) -> (Point, Point) {
    bar.1 .2 = bar.1 .2 - (bar.0 .2 - z);
    bar.0 .2 = z;

    bar
}

fn bar_crash(a: (Point, Point), b: (Point, Point)) -> bool {
    let mut crash_x = false;
    for x in b.0 .0..b.1 .0 + 1 {
        if a.0 .0 <= x && x <= a.1 .0 {
            crash_x = true;
            break;
        }
    }

    let mut crash_y = false;
    for y in b.0 .1..b.1 .1 + 1 {
        if a.0 .1 <= y && y <= a.1 .1 {
            crash_y = true;
            break;
        }
    }

    let mut crash_z = false;
    for z in b.0 .2..b.1 .2 + 1 {
        if a.0 .2 <= z && z <= a.1 .2 {
            crash_z = true;
            break;
        }
    }

    crash_x && crash_y && crash_z
}

fn bar_crash_2d(a: (Point, Point), b: (Point, Point)) -> bool {
    let mut crash_x = false;
    for x in b.0 .0..b.1 .0 + 1 {
        if a.0 .0 <= x && x <= a.1 .0 {
            crash_x = true;
            break;
        }
    }

    let mut crash_y = false;
    for y in b.0 .1..b.1 .1 + 1 {
        if a.0 .1 <= y && y <= a.1 .1 {
            crash_y = true;
            break;
        }
    }

    crash_x && crash_y
}
