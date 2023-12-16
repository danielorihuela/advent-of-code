use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let seeds_ranges = seeds_ranges(sections[0]);
    let transformation_maps = sections
        .into_iter()
        .skip(1)
        .map(relations)
        .collect::<Vec<Vec<(u128, u128, u128)>>>();

    let mut min = u128::MAX;
    for &range in seeds_ranges.iter() {
        let mut seed_ranges = vec![range];
        for transformation_map in transformation_maps.iter() {
            let mut new_seed_ranges = vec![];
            for &(x0, xf) in seed_ranges.iter() {
                let new_ranges_len = new_seed_ranges.len();
                for &(d0, s0, length) in transformation_map.iter() {
                    let sf = s0 + length - 1;
                    let df = d0 + length - 1;
                    // overlap (x0, sf)
                    if s0 < x0 && x0 < sf && sf < xf {
                        new_seed_ranges.push((x0 + d0 - s0, df));
                    }
                    // overlap (xs0, xf)
                    else if x0 < s0 && s0 < xf && xf < sf {
                        new_seed_ranges.push((d0, xf + d0 - s0));
                    }
                    // source range inside seed range
                    else if x0 < s0 && sf < xf {
                        new_seed_ranges.push((d0, df));
                    }
                    // seed range inside source range
                    else if s0 <= x0 && xf <= sf {
                        new_seed_ranges.push((x0 + d0 - s0, xf + d0 - s0));
                    }
                }
                // no intersection
                if new_seed_ranges.len() == new_ranges_len {
                    new_seed_ranges.push((x0, xf));
                }
            }
            seed_ranges = new_seed_ranges;
        }
        let current_min = seed_ranges.iter().map(|(a, _)| a).min().unwrap();
        min = min.min(*current_min);
    }
    println!("{}", min);
}

fn seeds_ranges(seed_ranges: &str) -> Vec<(u128, u128)> {
    let values = string_to_numbers(seed_ranges.split(": ").last().unwrap());
    let mut seeds = vec![];
    for data in values.windows(2).step_by(2) {
        seeds.push((data[0], data[0] + data[1] - 1));
    }

    seeds
}

fn string_to_numbers(line: &str) -> Vec<u128> {
    line.split_whitespace()
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>()
}

fn relations(section: &str) -> Vec<(u128, u128, u128)> {
    let mut relations = section
        .split("\n")
        .skip(1)
        .map(string_to_numbers)
        .map(|values| (values[0], values[1], values[2]))
        .collect::<Vec<(u128, u128, u128)>>();
    relations.sort_by(|a, b| a.1.cmp(&b.1));

    relations
}
