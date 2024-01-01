use std::fs;

type Point2d = (f64, f64);
type Speed2d = (f64, f64);
type LineEquation2d = (f64, f64);

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let data = input
        .lines()
        .map(|line| {
            let values = |data: &str| -> Vec<f64> {
                data.split(',')
                    .map(|x| x.trim())
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect()
            };
            let (point, speed) = line.split_once(" @ ").unwrap();
            let (point, speed) = (values(point), values(speed));

            ((point[0], point[1]), (speed[0], speed[1]))
        })
        .collect::<Vec<(Point2d, Speed2d)>>();

    let mut result = 0;
    for i in 0..data.len() {
        let (point_a, speed_a) = data[i];
        for j in i + 1..data.len() {
            let (point_b, speed_b) = data[j];
            let (m_a, c_a) = line_equation_2d(point_a, speed_a);
            let (m_b, c_b) = line_equation_2d(point_b, speed_b);
            if m_a != m_b && c_a != c_b {
                let (x, y) = intersection((m_a, c_a), (m_b, c_b));

                if (x < point_a.0 && speed_a.0 > 0.0)
                    || (x > point_a.0 && speed_a.0 < 0.0)
                    || (x < point_b.0 && speed_b.0 > 0.0)
                    || (x > point_b.0 && speed_b.0 < 0.0)
                {
                    continue;
                }

                let (min, max) = (200000000000000f64, 400000000000000f64);
                if min <= x && x <= max && min <= y && y <= max {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}

fn line_equation_2d(point: Point2d, speed: Speed2d) -> LineEquation2d {
    let m = speed.1 / speed.0;
    let c = point.1 - (m * point.0);

    (m, c)
}

fn intersection(equation_a: LineEquation2d, equation_b: LineEquation2d) -> Point2d {
    let ((a, c), (b, d)) = (equation_a, equation_b);
    let x = (d - c) / (a - b);
    let y = a * x + c;

    (x, y)
}
