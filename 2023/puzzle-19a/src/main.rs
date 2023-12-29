use std::{collections::HashMap, convert::Infallible, fs, str::FromStr};

#[derive(Debug)]
enum Rule {
    Complex(RatingPart, String, usize, Group),
    Simple(Group),
}

#[derive(Debug, Clone)]
enum Group {
    Arbitrary(String),
    Accepted,
    Rejected,
}

impl FromStr for Group {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Group::Accepted,
            "R" => Group::Rejected,
            arbitrary => Group::Arbitrary(arbitrary.to_string()),
        })
    }
}

#[derive(Debug)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn get_part(&self, part: &RatingPart) -> usize {
        match part {
            RatingPart::X => self.x,
            RatingPart::M => self.m,
            RatingPart::A => self.a,
            RatingPart::S => self.s,
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
enum RatingPart {
    X,
    M,
    A,
    S,
}

impl FromStr for RatingPart {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => RatingPart::X,
            "m" => RatingPart::M,
            "a" => RatingPart::A,
            "s" => RatingPart::S,
            _ => RatingPart::X,
        })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let (workflows, ratings) = (workflows(sections[0]), ratings(sections[1]));

    let mut result = 0;
    for rating in ratings {
        let mut next = Group::Arbitrary("in".to_string());
        loop {
            if let Group::Arbitrary(ref arbitrary) = next {
                for rule in workflows.get(arbitrary).unwrap() {
                    match rule {
                        Rule::Simple(group) => next = group.clone(),
                        Rule::Complex(part, operator, value, group) => {
                            let part = rating.get_part(&part);
                            if (operator == ">" && part > *value)
                                || (operator == "<" && part < *value)
                            {
                                next = group.clone();
                                break;
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        if let Group::Accepted = next {
            result += rating.sum();
        }
    }
    println!("{}", result);
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
                    RatingPart::from_str(category).unwrap(),
                    operator.to_string(),
                    value.parse::<usize>().unwrap(),
                    Group::from_str(group).unwrap(),
                )
            } else {
                Rule::Simple(Group::from_str(rule).unwrap())
            };
            rules.push(rule);
        }
        workflows.insert(name.to_string(), rules);
    }

    workflows
}

fn ratings(data: &str) -> Vec<Rating> {
    data.lines()
        .map(|line| line.split(['=', ',', '}']).collect::<Vec<&str>>())
        .map(|values| Rating {
            x: values[1].parse::<usize>().unwrap(),
            m: values[3].parse::<usize>().unwrap(),
            a: values[5].parse::<usize>().unwrap(),
            s: values[7].parse::<usize>().unwrap(),
        })
        .collect::<Vec<Rating>>()
}
