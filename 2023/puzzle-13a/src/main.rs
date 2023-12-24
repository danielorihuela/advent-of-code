use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0;
    for image in input.split("\n\n") {
        let pixels = image
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        if let Some(x) = vertical_reflection(&pixels) {
            result += x + 1;
        }
        if let Some(x) = horizontal_reflection(&pixels) {
            result += 100 * (x + 1);
        }
    }
    println!("{}", result);
}

fn vertical_reflection(pixels: &[Vec<char>]) -> Option<usize> {
    // left reflection
    for i in (1..pixels[0].len()).rev() {
        if !cmp_columns(pixels, 0, i) {
            continue;
        }

        if i % 2 == 0 {
            continue;
        }

        if (0..(i + 1) / 2).any(|j| !cmp_columns(pixels, j, i - j)) {
            continue;
        }

        return Some(i / 2);
    }

    // right reflection
    for i in 0..pixels[0].len() - 1 {
        if !cmp_columns(pixels, i, pixels[0].len() - 1) {
            continue;
        }

        if i % 2 == 0 {
            continue;
        }

        if (0..(pixels[0].len() - i) / 2)
            .any(|j| !cmp_columns(pixels, i + j, pixels[0].len() - 1 - j))
        {
            continue;
        }

        return Some(i + (pixels[0].len() - 1 - i) / 2);
    }

    None
}

fn cmp_columns(pixels: &[Vec<char>], a: usize, b: usize) -> bool {
    for i in 0..pixels.len() {
        if pixels[i][a] != pixels[i][b] {
            return false;
        }
    }

    true
}

fn horizontal_reflection(pixels: &[Vec<char>]) -> Option<usize> {
    // top reflection
    for i in (1..pixels.len()).rev() {
        if !cmp_rows(pixels, 0, i) {
            continue;
        }

        if i % 2 == 0 {
            continue;
        }

        if (0..(i + 1) / 2).any(|j| !cmp_rows(pixels, j, i - j)) {
            continue;
        }

        return Some(i / 2);
    }

    // bottom reflection
    for i in 0..pixels.len() - 1 {
        if !cmp_rows(pixels, i, pixels.len() - 1) {
            continue;
        }

        if i % 2 == 0 {
            continue;
        }

        if (0..(pixels.len() - i) / 2).any(|j| !cmp_rows(pixels, i + j, pixels.len() - 1 - j)) {
            continue;
        }

        return Some(i + (pixels.len() - 1 - i) / 2);
    }

    None
}

fn cmp_rows(pixels: &[Vec<char>], a: usize, b: usize) -> bool {
    for i in 0..pixels[0].len() {
        if pixels[a][i] != pixels[b][i] {
            return false;
        }
    }

    true
}
