use std::{
    collections::{BinaryHeap, HashMap},
    fs,
    str::Chars,
};

type Position = (usize, usize);
type Direction = (i8, i8);
type Cost = usize;

#[derive(PartialEq, Eq)]
struct Node {
    position: Position,
    direction: Direction,
    cost: Cost,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let to_digits = |line: Chars<'_>| line.map(|c| c.to_digit(10).unwrap()).collect();
    let matrix = input
        .lines()
        .map(str::chars)
        .map(to_digits)
        .collect::<Vec<Vec<u32>>>();

    let mut visited = HashMap::new();
    let mut pq = BinaryHeap::new();
    pq.push(Node {
        position: (0, 0),
        direction: (0, 0),
        cost: 0,
    });
    while !pq.is_empty() {
        let Node {
            position,
            direction,
            cost,
        } = pq.pop().unwrap();
        if visited.contains_key(&(position, direction)) {
            continue;
        }
        visited.insert((position, direction), cost);

        for (position, steps) in
            next_movements((matrix[0].len() - 1, matrix.len() - 1), position, direction)
        {
            pq.push(Node {
                position,
                direction: steps,
                cost: cost + matrix[position.0][position.1] as usize,
            });
        }
    }
    let shortest_path = visited
        .into_iter()
        .filter(|((position, _), _)| position == &(matrix[0].len() - 1, matrix.len() - 1))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    println!("{:?}", shortest_path.1);
}

fn next_movements(
    (max_x, max_y): (usize, usize),
    (x, y): Position,
    (sx, sy): Direction,
) -> Vec<(Position, Direction)> {
    let mut movements = vec![];
    if -4 < sx && sx < 0 && x > 0 {
        return vec![((x - 1, y), (sx - 1, 0))];
    }
    if 0 < sx && sx < 4 && x < max_x {
        return vec![((x + 1, y), (sx + 1, 0))];
    }
    if -4 < sy && sy < 0 && y > 0 {
        return vec![((x, y - 1), (0, sy - 1))];
    }
    if 0 < sy && sy < 4 && y < max_y {
        return vec![((x, y + 1), (0, sy + 1))];
    }

    if sx.abs() < 4 && sy.abs() < 4 && (sx != 0 || sy != 0) {
        return vec![];
    }

    if -10 < sx && sx <= 0 && x > 0 {
        movements.push(((x - 1, y), (sx - 1, 0)));
    }
    if 0 <= sx && sx < 10 && x < max_x {
        movements.push(((x + 1, y), (sx + 1, 0)));
    }
    if -10 < sy && sy <= 0 && y > 0 {
        movements.push(((x, y - 1), (0, sy - 1)));
    }
    if 0 <= sy && sy < 10 && y < max_y {
        movements.push(((x, y + 1), (0, sy + 1)));
    }

    movements
}
