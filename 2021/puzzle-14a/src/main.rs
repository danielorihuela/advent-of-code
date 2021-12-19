use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let polymer_template = read_polymer_template(&mut reader);
    let pair_insertion_rules = read_pair_insertion_rules(&mut reader);

    let mut polymer = polymer_template.chars().collect::<Vec<char>>();
    for _ in 0..10 {
        polymer = generate_new_polymer(&polymer, &pair_insertion_rules);
    }

    let elements_appearances = count_elements(&polymer);
    println!(
        "{}",
        *elements_appearances.values().max().unwrap()
            - *elements_appearances.values().min().unwrap()
    );
}

fn read_polymer_template(reader: &mut BufReader<File>) -> String {
    let mut polymer_template = String::new();
    reader.read_line(&mut polymer_template).unwrap();

    String::from(polymer_template.trim())
}

fn read_pair_insertion_rules(reader: &mut BufReader<File>) -> HashMap<(char, char), char> {
    let mut pair_insertion_rules: HashMap<(char, char), char> = HashMap::new();
    for line in reader.lines() {
        let words = line
            .unwrap()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        if words.len() == 3 {
            let first_word_chars = words[0].chars().collect::<Vec<char>>();
            pair_insertion_rules.insert(
                (first_word_chars[0], first_word_chars[1]),
                words[2].chars().next().unwrap(),
            );
        }
    }

    pair_insertion_rules
}

fn generate_new_polymer(
    polymer: &[char],
    pair_insertion_rules: &HashMap<(char, char), char>,
) -> Vec<char> {
    let mut new_polymer: Vec<char> = vec![polymer[0]];

    let all_but_last_char = polymer[..polymer.len() - 1].iter();
    let all_but_first_char = polymer[1..].iter();
    for (&a, &b) in all_but_last_char.zip(all_but_first_char) {
        let new = pair_insertion_rules.get(&(a, b)).unwrap();
        new_polymer.append(&mut vec![*new, b]);
    }

    new_polymer
}

fn count_elements(polymer: &[char]) -> HashMap<char, u16> {
    let mut element_count: HashMap<char, u16> = HashMap::new();
    for &letter in polymer {
        *element_count.entry(letter).or_insert(0) += 1;
    }

    element_count
}
