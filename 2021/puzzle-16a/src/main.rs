use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::num::ParseIntError;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut hexadecimal = String::new();
    reader.read_line(&mut hexadecimal).unwrap();

    let bits = hexadecimal
        .chars()
        .flat_map(|c| hex_to_bits(c))
        .collect::<String>();

    let mut i = 0;
    let mut sum_version = 0;
    while i + 3 < bits.len() {
        sum_version += bit_string_to_u16(&bits[i..i + 3]).unwrap();
        let type_id = bit_string_to_u16(&bits[i + 3..i + 6]).unwrap();
        i += 6;
        if type_id == 4 {
            let mut final_packet_reached = false;
            while !final_packet_reached {
                final_packet_reached = bit_string_to_u16(&bits[i..i + 1]).unwrap() == 0;
                i += 5;
            }
        } else {
            let length_type_id = bit_string_to_u16(&bits[i..i + 1]).unwrap();
            i += 1;
            i += match length_type_id {
                0 => 15,
                1 => 11,
                _ => 0,
            };
        }
    }
    println!("{}", sum_version);
}

fn hex_to_bits(hex: char) -> Option<String> {
    match hex {
        '0' => Some(String::from("0000")),
        '1' => Some(String::from("0001")),
        '2' => Some(String::from("0010")),
        '3' => Some(String::from("0011")),
        '4' => Some(String::from("0100")),
        '5' => Some(String::from("0101")),
        '6' => Some(String::from("0110")),
        '7' => Some(String::from("0111")),
        '8' => Some(String::from("1000")),
        '9' => Some(String::from("1001")),
        'A' => Some(String::from("1010")),
        'B' => Some(String::from("1011")),
        'C' => Some(String::from("1100")),
        'D' => Some(String::from("1101")),
        'E' => Some(String::from("1110")),
        'F' => Some(String::from("1111")),
        _ => None,
    }
}

fn bit_string_to_u16(bit_string: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(bit_string, 2)
}
