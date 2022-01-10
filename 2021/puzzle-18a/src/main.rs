use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut result = lines[0].clone();
    for i in 1..lines.len() {
        result = format!("[{},{}]", result.trim(), lines[i].trim());
        loop {
            if can_explode(&result).is_none() && can_split(&result).is_none() {
                break;
            }
            while let Some(target) = can_explode(&result) {
                result = explode(&result, target);
            }

            if let Some(target) = can_split(&result) {
                result = split(&result, target);
            }
        }
    }

    println!("{}", magnitude(&result));
}

fn can_explode(expression: &str) -> Option<(usize, usize)> {
    let mut depth = 0;
    let mut subexpression_start = 0;
    for (i, character) in expression.chars().enumerate() {
        depth += match character {
            '[' => 1,
            ']' => -1,
            _ => 0,
        };

        if subexpression_start != 0 && depth == 4 {
            return Some((subexpression_start, i));
        }
        if subexpression_start == 0 && depth == 5 {
            subexpression_start = i;
        }
    }

    None
}

fn explode(expr: &str, target: (usize, usize)) -> String {
    let subexpr = expr[target.0..target.1]
        .split(|c| c == '[' || c == ',')
        .collect::<Vec<&str>>();
    let first_value = subexpr[1].parse::<u64>().unwrap();
    let second_value = subexpr[2].parse::<u64>().unwrap();

    let mut left_number_indexes = (0, 0);
    for i in (0..target.0).rev() {
        if is_number(&expr[i..i + 1]) {
            left_number_indexes = (i, i + 1);
            if is_number(&expr[i - 1..i]) {
                left_number_indexes.0 = i - 1;
            }
            break;
        }
    }
    let mut left_side = format!("{}", &expr[..target.0]);
    if left_number_indexes != (0, 0) {
        let left_number = expr[left_number_indexes.0..left_number_indexes.1]
            .parse::<u64>()
            .unwrap()
            + first_value;
        left_side = format!(
            "{}{}{}",
            &expr[..left_number_indexes.0],
            left_number.to_string(),
            &expr[left_number_indexes.1..target.0]
        )
    }

    let mut right_number_indexes = (0, 0);
    for i in target.1..expr.len() {
        if is_number(&expr[i..i + 1]) {
            right_number_indexes = (i, i + 1);
            if is_number(&expr[i + 1..i + 2]) {
                right_number_indexes.1 = i + 2;
            }
            break;
        }
    }
    let mut right_side = format!("{}", &expr[target.1 + 1..]);
    if right_number_indexes != (0, 0) {
        let right_number = expr[right_number_indexes.0..right_number_indexes.1]
            .parse::<u64>()
            .unwrap()
            + second_value;
        right_side = format!(
            "{}{}{}",
            &expr[target.1 + 1..right_number_indexes.0],
            right_number.to_string(),
            &expr[right_number_indexes.1..]
        )
    }

    format!("{}0{}", left_side, right_side)
}

fn is_number(expr: &str) -> bool {
    expr != "[" && expr != "]" && expr != ","
}

fn can_split(expression: &str) -> Option<usize> {
    let characters = expression.chars().collect::<Vec<char>>();
    for i in 0..expression.len() {
        if is_number(&characters[i].to_string()) && is_number(&characters[i + 1].to_string()) {
            return Some(i);
        }
    }

    None
}

fn split(expression: &str, target: usize) -> String {
    let number = expression[target..target + 2].parse::<u64>().unwrap();

    let half = number / 2;
    let (a, b) = match number % 2 {
        0 => (half, half),
        1 => (half, half + 1),
        _ => (0, 0),
    };

    format!(
        "{}[{},{}]{}",
        &expression[..target],
        a,
        b,
        &expression[target + 2..]
    )
}

fn magnitude(expr: &str) -> String {
    let mut expr_copy = String::from(expr.clone());
    let re = Regex::new(r"\[(\d{1,}),(\d{1,})\]").unwrap();
    while &expr_copy[0..1] == "[" {
        if let Some(capture) = re.find(&expr_copy) {
            let splitted_pair: Vec<&str> = capture
                .as_str()
                .split(|c| c == '[' || c == ']' || c == ',')
                .collect();
            let (left, right) = (
                splitted_pair[1].parse::<u64>().unwrap(),
                splitted_pair[2].parse::<u64>().unwrap(),
            );
            expr_copy = format!(
                "{}{}{}",
                &expr_copy[..capture.start()],
                3 * left + 2 * right,
                &expr_copy[capture.end()..]
            );
        }
    }

    expr_copy
}
