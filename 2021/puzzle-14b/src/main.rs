use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let polymer_template = read_polymer_template(&mut reader);
    let pair_insertion_rules = read_pair_insertion_rules(&mut reader);

    let polymer = polymer_template.chars().collect::<Vec<char>>();
    let mut count_pairs: HashMap<(char, char), u64> = HashMap::new();
    for i in 0..polymer.len() - 1 {
        *count_pairs.entry((polymer[i], polymer[i + 1])).or_default() += 1;
    }
    for _ in 0..40 {
        let mut new_count_pairs: HashMap<(char, char), u64> = HashMap::new();
        for (&(a, b), value) in count_pairs.iter() {
            *new_count_pairs
                .entry((a, *pair_insertion_rules.get(&(a, b)).unwrap()))
                .or_default() += value;
            *new_count_pairs
                .entry((*pair_insertion_rules.get(&(a, b)).unwrap(), b))
                .or_default() += value;
        }
        count_pairs = new_count_pairs;
    }

    let mut count_elements: HashMap<char, u64> = HashMap::new();
    for (&(a, _), value) in count_pairs.iter() {
        *count_elements.entry(a).or_default() += value;
    }
    *count_elements.entry(*polymer.last().unwrap()).or_default() += 1;

    println!(
        "{}",
        *count_elements.values().max().unwrap() - *count_elements.values().min().unwrap()
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
