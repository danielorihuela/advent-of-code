use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut entry_points = vec![];
    for i in 0..matrix[0].len() as i8 {
        entry_points.push(((i, -1), (0, 1)));
        entry_points.push(((i, matrix.len() as i8), (0, -1)));
    }
    for i in 0..matrix.len() as i8 {
        entry_points.push(((-1, i), (1, 0)));
        entry_points.push(((matrix[0].len() as i8, i), (-1, 0)));
    }

    let result = entry_points
        .into_iter()
        .map(|entry_point| {
            let mut beams = vec![entry_point];
            let mut visited = HashSet::new();
            while !beams.is_empty() {
                let next_beams = beams
                    .into_iter()
                    .filter(|&((x0, y0), (sx, sy))| {
                        !((sx == -1 && x0 == 0)
                            || (sx == 1 && x0 == matrix[0].len() as i8 - 1)
                            || (sy == -1 && y0 == 0)
                            || (sy == 1 && y0 == matrix.len() as i8 - 1))
                    })
                    .flat_map(|((x0, y0), (sx, sy))| {
                        let (xf, yf) = (x0 + sx, y0 + sy);
                        match (sx, sy, matrix[yf as usize][xf as usize]) {
                            (-1, 0, '|') | (1, 0, '|') => {
                                vec![((xf, yf), (0, -1)), ((xf, yf), (0, 1))]
                            }
                            (0, -1, '-') | (0, 1, '-') => {
                                vec![((xf, yf), (-1, 0)), ((xf, yf), (1, 0))]
                            }
                            (1, 0, '\\') | (-1, 0, '/') => vec![((xf, yf), (0, 1))],
                            (-1, 0, '\\') | (1, 0, '/') => vec![((xf, yf), (0, -1))],
                            (0, 1, '\\') | (0, -1, '/') => vec![((xf, yf), (1, 0))],
                            (0, -1, '\\') | (0, 1, '/') => vec![((xf, yf), (-1, 0))],
                            (-1, 0, '-') | (1, 0, '-') => vec![((xf, yf), (sx, sy))],
                            (0, -1, '|') | (0, 1, '|') => vec![((xf, yf), (sx, sy))],
                            (_, _, '.') => vec![((xf, yf), (sx, sy))],
                            (_, _, _) => vec![((xf, yf), (sx, sy))],
                        }
                    });

                beams = next_beams
                    .filter(|&element| !visited.contains(&element))
                    .collect::<Vec<((i8, i8), (i8, i8))>>();
                for &beam in beams.iter() {
                    visited.insert(beam);
                }
            }

            visited
                .iter()
                .map(|(pos, _)| pos)
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap();
    println!("\n{}", result);
}
