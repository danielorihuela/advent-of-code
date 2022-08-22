use std::fs::File;
use std::io::{prelude::*, BufReader};

struct BingoGrid {
    grid_values: Vec<Vec<String>>,
    grid_selected: Vec<Vec<bool>>,
}

impl BingoGrid {
    fn new(grid_values: Vec<Vec<String>>) -> Self {
        Self {
            grid_values,
            grid_selected: vec![vec![false; 5]; 5],
        }
    }

    fn mark_if_present(self: &mut Self, value: &str) {
        for (i, j) in cartesian_product(0, 5) {
            self.grid_selected[i][j] |= self.grid_values[i][j] == value;
        }
    }

    fn has_complete_column(self: &Self) -> bool {
        for i in 0..5 {
            if self.grid_selected[i][0]
                && self.grid_selected[i][1]
                && self.grid_selected[i][2]
                && self.grid_selected[i][3]
                && self.grid_selected[i][4]
            {
                return true;
            }
        }

        false
    }

    fn has_complete_row(self: &Self) -> bool {
        for i in 0..5 {
            if self.grid_selected[0][i]
                && self.grid_selected[1][i]
                && self.grid_selected[2][i]
                && self.grid_selected[3][i]
                && self.grid_selected[4][i]
            {
                return true;
            }
        }

        false
    }

    fn sum_unmarked_numbers(self: &Self) -> i32 {
        let mut total = 0;
        for (i, j) in cartesian_product(0, 5) {
            total +=
                self.grid_values[i][j].parse::<i32>().unwrap() * !self.grid_selected[i][j] as i32;
        }

        total
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let drawed_numbers = read_drawed_numbers(&mut reader);

    let mut grids: Vec<BingoGrid> = vec![];
    while let Ok(value) = read_empty_line(&mut reader) {
        if value == 0 {
            break;
        }
        let bingo_grid = read_bingo_grid(&mut reader);
        grids.push(BingoGrid::new(bingo_grid));
    }

    let mut last_drawn_number = -1;
    let mut board_sum_unmarked_numbers = -1;
    let mut i = 0;
    while last_drawn_number == -1 && i != drawed_numbers.len() {
        let current_draw_number = drawed_numbers[i].parse::<i32>().unwrap();
        let mut j = 0;
        while board_sum_unmarked_numbers == -1 && j != grids.len() {
            grids[j].mark_if_present(&drawed_numbers[i]);
            if grids[j].has_complete_row() || grids[j].has_complete_column() {
                last_drawn_number = current_draw_number;
                board_sum_unmarked_numbers = grids[j].sum_unmarked_numbers();
            }
            j += 1;
        }
        i += 1;
    }
    println!("{}", last_drawn_number * board_sum_unmarked_numbers);
}

fn read_drawed_numbers(reader: &mut BufReader<std::fs::File>) -> Vec<String> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split(",")
        .map(|value| String::from(value.trim()))
        .collect()
}

fn read_empty_line(reader: &mut BufReader<std::fs::File>) -> Result<usize, std::io::Error> {
    let mut line = String::new();
    reader.read_line(&mut line)
}

fn read_bingo_grid(reader: &mut BufReader<std::fs::File>) -> Vec<Vec<String>> {
    let mut grid_values: Vec<Vec<String>> = vec![];
    for _ in 0..5 {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        grid_values.push(
            line.split_whitespace()
                .map(|value| String::from(value))
                .collect(),
        );
    }

    grid_values
}

fn cartesian_product(from: usize, to: usize) -> impl Iterator<Item = (usize, usize)> {
    (from..to).flat_map(move |a| (from..to).map(move |b| (a, b)))
}
