use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0u128;
    for line in input.lines() {
        let data = line.split_whitespace().collect::<Vec<&str>>();
        let records = data[0].chars().collect::<Vec<char>>();
        let groups = data[1]
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        result += count(&records, &groups, 0);
    }
    println!("{}", result);
}

fn count(records: &[char], groups: &[u8], num_hashtags: u8) -> u128 {
    match (records, groups) {
        ([], []) => return 1,
        ([], groups) if groups.len() == 1 && groups[0] == num_hashtags => return 1,
        ([], _) => return 0,
        (record, []) if record.iter().any(|c| c == &'#') => return 0,
        (_, []) => return 1,
        _ => (),
    }

    match (records[0], num_hashtags) {
        ('?', _) => {
            count(&[&['.'], &records[1..]].concat(), groups, num_hashtags)
                + count(&[&['#'], &records[1..]].concat(), groups, num_hashtags)
        }
        ('#', _) => count(&records[1..], groups, num_hashtags + 1),
        ('.', 0) => count(&records[1..], groups, num_hashtags),
        ('.', x) if x == groups[0] => count(&records[1..], &groups[1..], 0),
        ('.', _) => 0,
        (_, _) => 0,
    }
}
