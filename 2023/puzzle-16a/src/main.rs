use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut beams = vec![((-1i8, 0i8), (1i8, 0i8))];
    let mut visited = HashSet::new();
    let mut previous_step_visited = usize::MAX;
    while visited.len() != previous_step_visited {
        previous_step_visited = visited.len();
        let mut next_beams = vec![];
        for current_beam in beams.iter() {
            let &((x0, y0), (sx, sy)) = current_beam;
            if (sx == -1 && x0 == 0)
                || (sx == 1 && x0 == matrix[0].len() as i8 - 1)
                || (sy == -1 && y0 == 0)
                || (sy == 1 && y0 == matrix.len() as i8 - 1)
            {
                continue;
            }

            let (xf, yf) = (x0 + sx, y0 + sy);
            visited.insert(((xf, yf), (sx, sy)));

            let new_beams = match (sx, sy, matrix[yf as usize][xf as usize]) {
                (-1, 0, '|') | (1, 0, '|') => vec![((xf, yf), (0, -1)), ((xf, yf), (0, 1))],
                (0, -1, '-') | (0, 1, '-') => vec![((xf, yf), (-1, 0)), ((xf, yf), (1, 0))],
                (1, 0, '\\') | (-1, 0, '/') => vec![((xf, yf), (0, 1))],
                (-1, 0, '\\') | (1, 0, '/') => vec![((xf, yf), (0, -1))],
                (0, 1, '\\') | (0, -1, '/') => vec![((xf, yf), (1, 0))],
                (0, -1, '\\') | (0, 1, '/') => vec![((xf, yf), (-1, 0))],
                (-1, 0, '-') | (1, 0, '-') => vec![((xf, yf), (sx, sy))],
                (0, -1, '|') | (0, 1, '|') => vec![((xf, yf), (sx, sy))],
                (_, _, '.') => vec![((xf, yf), (sx, sy))],
                (_, _, _) => vec![((xf, yf), (sx, sy))],
            };
            new_beams.into_iter().for_each(|beam| next_beams.push(beam));
        }

        beams = next_beams;
    }

    let visited = visited.iter().map(|(pos, _)| pos).collect::<HashSet<_>>();
    println!("{}", visited.len());
}
