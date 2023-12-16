use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = seeds(sections[0]);
    let transformation_maps = sections
        .into_iter()
        .skip(1)
        .map(transformation_triplets)
        .collect::<Vec<Vec<(u64, u64, u64)>>>();

    let result = seeds
        .into_iter()
        .map(|seed| {
            transformation_maps
                .iter()
                .fold(seed, |source, transformation_map| {
                    transform(transformation_map, source)
                })
        })
        .min();
    println!("{}", result.unwrap());
}

fn seeds(seeds: &str) -> Vec<u64> {
    string_to_numbers(seeds.split(":").last().unwrap())
}

fn string_to_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn transformation_triplets(section: &str) -> Vec<(u64, u64, u64)> {
    section
        .split("\n")
        .skip(1)
        .map(string_to_numbers)
        .map(|values| (values[0], values[1], values[2]))
        .collect::<Vec<(u64, u64, u64)>>()
}

fn transform(transformation_map: &[(u64, u64, u64)], source: u64) -> u64 {
    for transformation in transformation_map {
        if transformation.1 <= source && source <= transformation.1 + transformation.2 {
            return source + transformation.0 - transformation.1;
        }
    }

    source
}
