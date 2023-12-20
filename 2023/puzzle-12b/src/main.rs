use memoize::memoize;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0u128;
    for line in input.lines() {
        let data = line.split_whitespace().collect::<Vec<&str>>();
        let records = data[0].chars().collect::<Vec<char>>();
        let records = [
            records.clone(),
            vec!['?'],
            records.clone(),
            vec!['?'],
            records.clone(),
            vec!['?'],
            records.clone(),
            vec!['?'],
            records.clone(),
        ]
        .concat();
        let groups = data[1]
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let groups = [
            groups.clone(),
            groups.clone(),
            groups.clone(),
            groups.clone(),
            groups.clone(),
        ]
        .concat();

        result += count(records, groups, 0);
    }
    println!("{}", result);
}

#[memoize]
fn count(records: Vec<char>, groups: Vec<u8>, num_hashtags: u8) -> u128 {
    match (&records, &groups) {
        (r, g) if r.is_empty() && g.is_empty() => return 1,
        (r, groups) if r.is_empty() && groups.len() == 1 && groups[0] == num_hashtags => return 1,
        (r, _) if r.is_empty() => return 0,
        (record, g) if g.is_empty() && record.iter().any(|c| c == &'#') => return 0,
        (_, g) if g.is_empty() => return 1,
        _ => (),
    }

    match (records[0], num_hashtags) {
        ('?', _) => {
            count(
                [&['.'], &records[1..]].concat().to_vec(),
                groups.clone(),
                num_hashtags,
            ) + count(
                [&['#'], &records[1..]].concat().to_vec(),
                groups,
                num_hashtags,
            )
        }
        ('#', _) => count(records[1..].to_vec(), groups, num_hashtags + 1),
        ('.', 0) => count(records[1..].to_vec(), groups, num_hashtags),
        ('.', x) if x == groups[0] => count(records[1..].to_vec(), groups[1..].to_vec(), 0),
        ('.', _) => 0,
        (_, _) => 0,
    }
}
