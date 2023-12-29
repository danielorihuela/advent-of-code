use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Rule {
    Complex(usize, String, usize, String),
    Simple(String),
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let workflows = workflows(sections[0]);

    let ranges = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    let total = possible_combinations(ranges, &workflows, "in".to_string());
    println!("{}", total);
}

fn workflows(data: &str) -> HashMap<String, Vec<Rule>> {
    let mut workflows = HashMap::new();

    for line in data.lines() {
        let (name, raw_rules) = line[0..line.len() - 1].split_once("{").unwrap();
        let mut rules = vec![];
        for rule in raw_rules.split(",").collect::<Vec<&str>>() {
            let rule = if let Some((check, group)) = rule.split_once(":") {
                let operator = &check[1..2];
                let (category, value) = check.split_once(['<', '>']).unwrap();
                Rule::Complex(
                    part_to_value(category),
                    operator.to_string(),
                    value.parse::<usize>().unwrap(),
                    group.to_string(),
                )
            } else {
                Rule::Simple(rule.to_string())
            };
            rules.push(rule);
        }
        workflows.insert(name.to_string(), rules);
    }

    workflows
}

fn part_to_value(part: &str) -> usize {
    match part {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => 0,
    }
}

fn possible_combinations(
    mut ranges: Vec<(i128, i128)>,
    workflows: &HashMap<String, Vec<Rule>>,
    group: String,
) -> i128 {
    if group == "R" {
        return 0;
    }
    if group == "A" {
        return ranges.iter().map(|(min, max)| max - min + 1).product();
    }

    let mut total = 0;
    for i in 0..workflows[&group].len() {
        match &workflows[&group][i] {
            Rule::Simple(inner_group) => {
                total += possible_combinations(ranges.clone(), workflows, inner_group.clone())
            }
            Rule::Complex(part, operator, value, inner_group) => {
                let mut new_ranges = ranges.clone();
                if operator == "<" {
                    new_ranges[part.clone() as usize].1 = *value as i128 - 1;
                    ranges[part.clone() as usize].0 = *value as i128;
                } else {
                    new_ranges[part.clone() as usize].0 = *value as i128 + 1;
                    ranges[part.clone() as usize].1 = *value as i128;
                }
                total += possible_combinations(new_ranges, workflows, inner_group.clone());
            }
        }
    }

    total
}
