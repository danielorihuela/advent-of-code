use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0;
    for image in input.split("\n\n") {
        let pixels = image
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        if let Some(x) = horizontal_reflection(&pixels) {
            result += 100 * (x + 1);
        }
        if let Some(x) = vertical_reflection(&pixels) {
            result += x + 1;
        }
    }
    println!("{}", result);
}

fn vertical_reflection(pixels: &[Vec<char>]) -> Option<usize> {
    // left reflection
    'outer: for i in (1..pixels[0].len()).step_by(2).rev() {
        let mut count = 0;
        for j in 0..(i + 1) / 2 {
            let diff = cmp_columns(pixels, j, i - j);
            if diff > 1 {
                continue 'outer;
            } else if diff == 1 {
                count += 1;
            }
        }
        if count != 1 {
            continue;
        }

        return Some(i / 2);
    }

    // right reflection
    'outer: for i in (1..pixels[0].len() - 1).step_by(2) {
        let mut count = 0;
        for j in 0..(pixels[0].len() - i) / 2 {
            let diff = cmp_columns(pixels, i + j, pixels[0].len() - 1 - j);
            if diff > 1 {
                continue 'outer;
            } else if diff == 1 {
                count += 1;
            }
        }
        if count != 1 {
            continue;
        }

        return Some(i + (pixels[0].len() - 1 - i) / 2);
    }

    None
}

fn cmp_columns(pixels: &[Vec<char>], a: usize, b: usize) -> usize {
    let mut count = 0;
    for i in 0..pixels.len() {
        if pixels[i][a] != pixels[i][b] {
            count += 1;
        }
    }

    count
}

fn horizontal_reflection(pixels: &[Vec<char>]) -> Option<usize> {
    // top reflection
    'outer: for i in (1..pixels.len()).step_by(2).rev() {
        let mut count = 0;
        for j in 0..(i + 1) / 2 {
            let diff = cmp_rows(pixels, j, i - j);
            if diff > 1 {
                continue 'outer;
            } else if diff == 1 {
                count += 1;
            }
        }
        if count != 1 {
            continue;
        }

        return Some(i / 2);
    }

    // bottom reflection
    'outer: for i in (1..pixels.len() - 1).step_by(2) {
        let mut count = 0;
        for j in 0..(pixels.len() - i) / 2 {
            let diff = cmp_rows(pixels, i + j, pixels.len() - 1 - j);
            if diff > 1 {
                continue 'outer;
            } else if diff == 1 {
                count += 1;
            }
        }
        if count != 1 {
            continue;
        }

        return Some(i + (pixels.len() - 1 - i) / 2);
    }

    None
}

fn cmp_rows(pixels: &[Vec<char>], a: usize, b: usize) -> usize {
    let mut count = 0;
    for i in 0..pixels[0].len() {
        if pixels[a][i] != pixels[b][i] {
            count += 1;
        }
    }

    count
}
