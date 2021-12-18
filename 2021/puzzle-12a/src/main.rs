use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut small_caves: HashSet<String> = HashSet::new();
    let mut cave_to_caves: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let path = line
            .unwrap()
            .split("-")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        update_path(&mut cave_to_caves, &path[0], &path[1]);
        update_path(&mut cave_to_caves, &path[1], &path[0]);

        add_small_cave(&mut small_caves, &path[0]);
        add_small_cave(&mut small_caves, &path[1]);
    }

    let origin = String::from("start");
    let mut all_paths: Vec<Vec<String>> = vec![];
    paths_to_end(cave_to_caves, &small_caves, vec![origin], &mut all_paths);
    println!("{}", all_paths.len());
}

fn update_path(cave_to_caves: &mut HashMap<String, Vec<String>>, origin: &str, destination: &str) {
    if let Some(caves) = cave_to_caves.get_mut(origin) {
        caves.push(String::from(destination));
    } else {
        cave_to_caves.insert(String::from(origin), vec![String::from(destination)]);
    }
}

fn add_small_cave(small_caves: &mut HashSet<String>, word: &str) {
    if word != "end" && is_lowercase(word) {
        small_caves.insert(String::from(word));
    }
    if word != "end" && is_lowercase(word) {
        small_caves.insert(String::from(word));
    }
}

fn is_lowercase(word: &str) -> bool {
    for letter in word.chars() {
        if (letter as u32) < 96 {
            return false;
        }
    }

    true
}

fn paths_to_end(
    cave_to_caves: HashMap<String, Vec<String>>,
    small_caves: &HashSet<String>,
    path_to_here: Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) {
    if invalid_path(small_caves, &path_to_here) {
        return;
    }

    let origin = path_to_here[path_to_here.len() - 1].clone();
    let destinations = cave_to_caves.get(&origin).unwrap();

    for destination in destinations {
        let mut new_path = path_to_here.clone();
        new_path.push(String::from(destination));
        if destination == "end" {
            all_paths.push(new_path);
        } else {
            let mut cave_to_caves_copy = cave_to_caves.clone();
            let mut new_destinations = destinations.clone();
            new_destinations.retain(|x| *x != *destination);
            cave_to_caves_copy.insert(String::from(&origin), new_destinations);
            paths_to_end(cave_to_caves_copy, small_caves, new_path, all_paths);
        }
    }
}

fn invalid_path(small_caves: &HashSet<String>, path_to_here: &[String]) -> bool {
    for small_cave in small_caves {
        let repeated_small_cave = path_to_here
            .iter()
            .filter(|&cave| *cave == *small_cave)
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
            .len();
        if repeated_small_cave > 1 {
            return true;
        }
    }

    false
}
