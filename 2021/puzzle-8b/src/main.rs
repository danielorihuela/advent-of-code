use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let (mut input, output) = get_input_and_output(&line.unwrap());
        for word in &mut input {
            word.sort_by(|a, b| a.cmp(b));
        }
        input.sort_by(|a, b| a.len().cmp(&b.len()));

        let digit_to_position = calculate_digit_to_position_relations(&input);
        let segment_to_segment = calculate_segments_relations(input, digit_to_position);

        let digit = output
            .iter()
            .map(|word| word_to_digit_as_char(&word, &segment_to_segment))
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        total += digit;
    }

    println!("{}", total);
}

fn get_input_and_output(line: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let data = line
        .split("|")
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();
    (data[0].clone(), data[1].clone())
}

fn calculate_digit_to_position_relations(digit_segments: &Vec<Vec<char>>) -> HashMap<u8, u8> {
    let mut digit_to_position: HashMap<u8, u8> = HashMap::new();
    digit_to_position.insert(1, 0);
    digit_to_position.insert(4, 2);
    digit_to_position.insert(7, 1);
    digit_to_position.insert(8, 9);

    for i in 6..9 {
        if is_subarray(&digit_segments[2], &digit_segments[i]) {
            digit_to_position.insert(9, i as u8);
        }
    }

    for i in 3..6 {
        let mut num_subarrays = 0;
        for j in 6..9 {
            if !is_subarray(&digit_segments[i], &digit_segments[j]) {
                continue;
            }

            num_subarrays += 1;
            if j as u8 != *digit_to_position.get(&9).unwrap() {
                digit_to_position.insert(5, i as u8);
                digit_to_position.insert(6, j as u8);
            }
        }
        match num_subarrays {
            0 => digit_to_position.insert(2, i as u8),
            1 => digit_to_position.insert(3, i as u8),
            _ => None,
        };
    }

    let zero_position =
        6 + 7 + 8 - digit_to_position.get(&9).unwrap() - digit_to_position.get(&6).unwrap();
    digit_to_position.insert(0, zero_position);

    digit_to_position
}

fn is_subarray(this: &[char], of: &[char]) -> bool {
    for letter in this {
        if !of.contains(letter) {
            return false;
        }
    }

    true
}

fn calculate_segments_relations(
    digit_segments: Vec<Vec<char>>,
    digit_to_position: HashMap<u8, u8>,
) -> HashMap<char, char> {
    let mut segment_to_segment: HashMap<char, char> = HashMap::new();

    let zero = &digit_segments[*digit_to_position.get(&0).unwrap() as usize];
    let one = &digit_segments[*digit_to_position.get(&1).unwrap() as usize];
    let two = &digit_segments[*digit_to_position.get(&2).unwrap() as usize];
    let three = &digit_segments[*digit_to_position.get(&3).unwrap() as usize];
    let four = &digit_segments[*digit_to_position.get(&4).unwrap() as usize];
    let six = &digit_segments[*digit_to_position.get(&6).unwrap() as usize];
    let seven = &digit_segments[*digit_to_position.get(&7).unwrap() as usize];
    let eight = &digit_segments[*digit_to_position.get(&8).unwrap() as usize];
    let nine = &digit_segments[*digit_to_position.get(&9).unwrap() as usize];

    let a = first_different_char(&one, &seven);
    segment_to_segment.insert(a, 'a');

    let b = first_different_char(&three, &nine);
    segment_to_segment.insert(b, 'b');

    let c = first_different_char(&six, &eight);
    segment_to_segment.insert(c, 'c');

    let d = first_different_char(&zero, &eight);
    segment_to_segment.insert(d, 'd');

    let e = first_different_char(&nine, &eight);
    segment_to_segment.insert(e, 'e');

    let mut temp = two.clone();
    temp.append(&mut vec![b]);
    temp.sort_by(|a, b| a.cmp(b));
    let f = first_different_char(&temp, &eight);
    segment_to_segment.insert(f, 'f');

    let mut temp = four.clone();
    temp.append(&mut vec![a]);
    temp.sort_by(|a, b| a.cmp(b));
    let g = first_different_char(&temp, &nine);
    segment_to_segment.insert(g, 'g');

    segment_to_segment
}

fn first_different_char(smaller_word: &[char], larger_word: &[char]) -> char {
    for i in 0..smaller_word.len() {
        if smaller_word[i] != larger_word[i] {
            return larger_word[i];
        }
    }

    larger_word[smaller_word.len()]
}

fn word_to_digit_as_char(word: &[char], segment_to_segment: &HashMap<char, char>) -> char {
    let mut digit = word
        .iter()
        .map(|letter| *segment_to_segment.get(&letter).unwrap())
        .collect::<Vec<char>>();
    digit.sort_by(|a, b| a.cmp(b));
    let digit: String = digit.iter().collect();
    digit_representation_to_char(&digit)
}

fn digit_representation_to_char(digit: &str) -> char {
    match digit {
        "abcefg" => '0',
        "cf" => '1',
        "acdeg" => '2',
        "acdfg" => '3',
        "bcdf" => '4',
        "abdfg" => '5',
        "abdefg" => '6',
        "acf" => '7',
        "abcdefg" => '8',
        "abcdfg" => '9',
        _ => ' ',
    }
}
