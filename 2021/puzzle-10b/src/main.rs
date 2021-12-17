use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let all_lines = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut scores: Vec<u64> = vec![];
    for line in all_lines {
        let mut score = 0u64;
        if let Some(mut symbols_in_queue) = corrupted_line(line) {
            while let Some(last_character) = symbols_in_queue.pop() {
                score *= 5;
                score += syntax_completion_score(last_character) as u64;
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{:?}", scores[scores.len() / 2]);
}

fn corrupted_line(symbols: Vec<char>) -> Option<Vec<char>> {
    let mut queue: Vec<char> = vec![];
    for symbol in symbols {
        if open_character(symbol) {
            queue.push(symbol);
        } else if opposite_character(symbol) == queue[queue.len() - 1] {
            queue.pop();
        } else {
            return None;
        }
    }

    Some(queue)
}

fn open_character(character: char) -> bool {
    match character {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn opposite_character(character: char) -> char {
    match character {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => ' ',
    }
}

fn syntax_completion_score(character: char) -> u16 {
    match character {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
