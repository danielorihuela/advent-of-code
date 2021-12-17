use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0u32;
    for line in reader.lines() {
        let symbols: Vec<char> = line.unwrap().chars().collect::<Vec<char>>();
        let mut queue: Vec<char> = vec![];
        for symbol in symbols {
            if open_character(symbol) {
                queue.push(symbol);
            } else if opposite_character(symbol) == queue[queue.len() - 1] {
                queue.pop();
            } else {
                score += syntax_error_score(symbol) as u32;
                break;
            }
        }
    }

    println!("{}", score);
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

fn syntax_error_score(character: char) -> u16 {
    match character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
