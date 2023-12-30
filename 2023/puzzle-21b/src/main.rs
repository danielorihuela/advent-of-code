use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut origin = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'S' {
                origin = (j, i);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(((origin.0, origin.1, 0i32, 0i32), 0usize));
    while !queue.is_empty() {
        let ((x, y, x_map, y_map), step) = queue.pop_front().unwrap();
        if visited.contains_key(&(x, y, x_map, y_map)) {
            continue;
        }
        visited.insert((x, y, x_map, y_map), step);

        if step == 328 {
            break;
        }

        let mut new_map = (x_map, y_map);
        let new_x = if x == 0 {
            new_map.0 -= 1;
            matrix[0].len() - 1
        } else {
            x - 1
        };
        if matrix[y][new_x] != '#' {
            queue.push_back(((new_x, y, new_map.0, new_map.1), step + 1));
        }

        let mut new_map = (x_map, y_map);
        let new_x = if x == matrix[0].len() - 1 {
            new_map.0 += 1;
            0
        } else {
            x + 1
        };
        if matrix[y][new_x] != '#' {
            queue.push_back(((new_x, y, new_map.0, new_map.1), step + 1));
        }

        let mut new_map = (x_map, y_map);
        let new_y = if y == 0 {
            new_map.1 -= 1;
            matrix.len() - 1
        } else {
            y - 1
        };
        if matrix[new_y][x] != '#' {
            queue.push_back(((x, new_y, new_map.0, new_map.1), step + 1));
        }

        let mut new_map = (x_map, y_map);
        let new_y = if y == matrix.len() - 1 {
            new_map.1 += 1;
            0
        } else {
            y + 1
        };
        if matrix[new_y][x] != '#' {
            queue.push_back(((x, new_y, new_map.0, new_map.1), step + 1));
        }
    }

    let points = vec![65, 65 + 131, 65 + 131 * 2];
    let mut visited_at_step = vec![];
    for step in points.iter() {
        let mut total = 0;
        for i in (step % 2..step + 1).into_iter().step_by(2) {
            total += visited.iter().filter(|(_, &step)| step == i).count();
        }
        visited_at_step.push(total);
    }
    println!("{:?}", points);
    println!("{:?}", visited_at_step);

    // Check explanation at This explanation is amazing https://www.youtube.com/watch?v=yVJ-J5Nkios
    // 1. Compute "diamond" points (`points` in the code)
    // 2. Compute garden plots reached at each opints (`visited_at_step` in the code)
    // 3. Compute quadratic equation with wolfram alpha https://www.wolframalpha.com/input?i=quadratic+fit+calculator&assumption=%7B%22F%22%2C+%22QuadraticFitCalculator%22%2C+%22data3x%22%7D+-%3E%22%7B0%2C+1%2C+2%7D%22&assumption=%7B%22F%22%2C+%22QuadraticFitCalculator%22%2C+%22data3y%22%7D+-%3E%22%7B3720%2C+33150%2C+91890%7D%22
    // 4. Solve the formula resulting from step 3 with x = (steps - 65) / 131. https://www.wolframalpha.com/input?i=3720+%2B+14775+x+%2B+14655+x%5E2+where+x+%3D+202300
    // 5. Result is 599763113936220
}
