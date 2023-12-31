use std::fs;

type Point = (i128, i128, i128);

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut bars = input
        .lines()
        .map(|line| line.split_once('~').unwrap())
        .map(|points| (str_to_point(points.0), str_to_point(points.1)))
        .collect::<Vec<(Point, Point)>>();
    bars.sort_by(|a, b| a.1 .2.cmp(&b.1 .2));

    let fallen_bars = wait_pieces_to_fall(bars).0;

    let mut total_falling_bars = 0;
    for (i, &bar) in fallen_bars.iter().enumerate() {
        let z = bar.1 .2;
        for &upper_bar in fallen_bars.iter().filter(|bar| bar.0 .2 == z + 1) {
            let mut crashes = fallen_bars
                .iter()
                .filter(|bar| bar.1 .2 == z)
                .filter(|&&bar| bar_crash_2d(bar, upper_bar) == true);
            if crashes.clone().count() == 1 && crashes.next().unwrap() == &bar {
                let mut bars = fallen_bars.clone();
                bars.remove(i);
                total_falling_bars += wait_pieces_to_fall(bars).1;
                break;
            }
        }
    }
    println!("{}", total_falling_bars);
}

fn str_to_point(data: &str) -> Point {
    let point = data
        .split(',')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    (point[0], point[1], point[2])
}

fn wait_pieces_to_fall(sorted_bars: Vec<(Point, Point)>) -> (Vec<(Point, Point)>, usize) {
    let mut count = 0;
    let mut fallen_bars = vec![];
    fallen_bars.push(move_bar_down(sorted_bars[0], 1));
    for &current_bar in sorted_bars.iter().skip(1) {
        let mut z = current_bar.0 .2;
        while z > 0 {
            z -= 1;
            if z == 0
                || fallen_bars
                    .iter()
                    .filter(|bar| bar.1 .2 == z)
                    .any(|&bar| bar_crash_2d(bar, current_bar))
            {
                if z + 1 != current_bar.0 .2 {
                    count += 1;
                }
                fallen_bars.push(move_bar_down(current_bar, z + 1));
                break;
            }
        }
    }

    (fallen_bars, count)
}

fn move_bar_down(mut bar: (Point, Point), z: i128) -> (Point, Point) {
    bar.1 .2 -= bar.0 .2 - z;
    bar.0 .2 = z;

    bar
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
