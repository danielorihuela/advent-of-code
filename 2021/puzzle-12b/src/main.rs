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
    paths_to_end(&cave_to_caves, &small_caves, vec![origin], &mut all_paths);
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
    if word != "start" && word != "end" && is_lowercase(word) {
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
    cave_to_caves: &HashMap<String, Vec<String>>,
    small_caves: &HashSet<String>,
    path_to_here: Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) {
    if invalid_path(&path_to_here) {
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
            paths_to_end(cave_to_caves, small_caves, new_path, all_paths);
        }
    }
}

fn invalid_path(path_to_here: &[String]) -> bool {
    let repetitions = path_to_here
        .iter()
        .filter(|&cave| *cave == "start")
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
        .len();
    if repetitions == 2 {
        return true;
    }

    let mut small_caves_appearances: HashMap<&str, u16> = HashMap::new();
    for cave in path_to_here {
        if is_lowercase(cave) {
            *small_caves_appearances.entry(cave).or_insert(0) += 1;
        }
    }

    let mut repeated_small_caves = 0;
    for (_, repetitions) in small_caves_appearances {
        if repetitions > 2 {
            return true;
        } else if repetitions > 1 {
            repeated_small_caves += 1;
        }
        if repeated_small_caves == 2 {
            return true;
        }
    }

    false
}
