use std::fs::File;
use std::io::{prelude::*, BufReader};

type PixelPosition = (i64, i64);

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let image_enhancement = &lines[0];

    let mut enhanced_image = lines
        .iter()
        .skip(2)
        .map(|x| x.clone())
        .collect::<Vec<Vec<char>>>();
    for i in 1..3 {
        let border_character = if i % 2 == 0 {
            image_enhancement[0]
        } else {
            '.'
        };
        let image_with_borders = add_border(&enhanced_image, border_character);
        enhanced_image = enhance_image(&image_with_borders, image_enhancement, border_character);
    }

    let total = num_lit_pixels(&enhanced_image);
    println!("{}", total);
}

fn add_border(image: &[Vec<char>], border_character: char) -> Vec<Vec<char>> {
    let horizontal_border = vec![vec![border_character; image[0].len() + 2]];
    let vertical_border = vec![border_character; 1];

    let new_image = image
        .iter()
        .map(|row| {
            [
                vertical_border.clone(),
                row.clone(),
                vertical_border.clone(),
            ]
            .concat()
        })
        .collect::<Vec<Vec<char>>>();

    [horizontal_border.clone(), new_image, horizontal_border].concat()
}

fn enhance_image(
    image: &[Vec<char>],
    image_enhancement: &[char],
    border_character: char,
) -> Vec<Vec<char>> {
    let enhance_row = |x: i64| {
        (0..image[0].len())
            .into_iter()
            .map(|y| output_pixel((x, y as i64), image, image_enhancement, border_character))
            .collect::<Vec<char>>()
    };

    (0..image.len())
        .into_iter()
        .map(|x| enhance_row(x as i64))
        .collect::<Vec<Vec<char>>>()
}

fn output_pixel(
    pixel_position: PixelPosition,
    image: &[Vec<char>],
    image_enhancement: &[char],
    border_character: char,
) -> char {
    let pixels_positions = pixel_area(pixel_position);
    let characters = pixels_value(image, &pixels_positions, border_character);
    let binary_string = binary_string(&characters);
    let binary = i64::from_str_radix(&binary_string, 2).unwrap();

    image_enhancement[binary as usize]
}

fn pixel_area(pixel_position: PixelPosition) -> Vec<PixelPosition> {
    let (min_x, min_y) = (pixel_position.0 - 1, pixel_position.1 - 1);
    let (max_x, max_y) = (pixel_position.0 + 1, pixel_position.1 + 1);

    (min_x..max_x + 1)
        .into_iter()
        .flat_map(|x| (min_y..max_y + 1).into_iter().map(move |y| (x, y)))
        .collect::<Vec<PixelPosition>>()
}

fn pixels_value(
    image: &[Vec<char>],
    pixels_positions: &[PixelPosition],
    border_character: char,
) -> Vec<char> {
    pixels_positions
        .iter()
        .map(|(x, y)| match image.get(*x as usize) {
            Some(chars) => *chars.get(*y as usize).unwrap_or(&border_character),
            None => border_character,
        })
        .collect::<Vec<char>>()
}

fn binary_string(characters: &[char]) -> String {
    characters
        .iter()
        .map(|&c| ((c == '#') as u8).to_string())
        .collect::<String>()
}

fn num_lit_pixels(image: &[Vec<char>]) -> u64 {
    (0..image.len())
        .into_iter()
        .flat_map(|x| {
            (0..image[0].len())
                .into_iter()
                .map(move |y| (image[x][y] == '#') as u64)
        })
        .fold(0, |total, next| total + next)
}
