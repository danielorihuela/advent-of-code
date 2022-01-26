use std::fs::File;
use std::io::{prelude::*, BufReader};

type State = Vec<Vec<char>>;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let initial_state = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut current_state = initial_state;
    let mut next_state: State = compute_next_state(&current_state);
    let mut i = 1;
    while current_state != next_state {
        current_state = next_state;
        next_state = compute_next_state(&current_state);
        i += 1;
    }
    println!("{}", i);
}

fn compute_next_state(current_state: &State) -> State {
    let width = current_state[0].len();
    let height = current_state.len();

    move_herd_down(
        &move_herd_right(current_state, width, height),
        width,
        height,
    )
}

fn move_herd_right(current_state: &State, width: usize, height: usize) -> State {
    let mut next_state = current_state.clone();
    for i in 0..width {
        for j in 0..height {
            if current_state[j][i] == '>' && current_state[j][(i + 1) % width] == '.' {
                next_state[j][i] = '.';
                next_state[j][(i + 1) % width] = '>';
            }
        }
    }

    next_state
}

fn move_herd_down(current_state: &State, width: usize, height: usize) -> State {
    let mut next_state = current_state.clone();
    for j in 0..height {
        for i in 0..width {
            if current_state[j][i] == 'v' && current_state[(j + 1) % height][i] == '.' {
                next_state[j][i] = '.';
                next_state[(j + 1) % height][i] = 'v';
            }
        }
    }

    next_state
}
