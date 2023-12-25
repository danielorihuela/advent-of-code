use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let boxes = input.split(",").fold(
        HashMap::<usize, Vec<(&str, usize)>>::new(),
        |mut acc, value| {
            if value.contains('=') {
                let values = value.split("=").collect::<Vec<&str>>();
                let (label, focal_length) = (values[0], values[1].parse::<usize>().unwrap());
                let key = hash(label);

                if let Some(box_data) = acc.get_mut(&key) {
                    if let Some(x) = box_data.iter_mut().find(|element| element.0 == label) {
                        x.1 = focal_length;
                    } else {
                        box_data.push((label, focal_length));
                    }
                } else {
                    acc.insert(key, vec![(label, focal_length)]);
                }
            } else if &value[value.len() - 1..] == "-" {
                let label = &value[..value.len() - 1];
                let key = hash(label);
                acc.entry(key).and_modify(|data| {
                    if let Some(i) = data
                        .iter()
                        .enumerate()
                        .find(|(_, element)| element.0 == label)
                        .map(|(i, _)| i)
                    {
                        data.remove(i);
                    }
                });
            }

            acc
        },
    );

    let result = boxes
        .iter()
        .filter(|box_data| !box_data.1.is_empty())
        .map(|(i, data)| {
            data.iter()
                .enumerate()
                .map(|(j, (_, k))| (i + 1) * (j + 1) * k)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{:?}", result);
}

fn hash(value: &str) -> usize {
    value
        .as_bytes()
        .iter()
        .fold(0, |acc, &value| (17 * (acc + value as usize)) % 256)
}
