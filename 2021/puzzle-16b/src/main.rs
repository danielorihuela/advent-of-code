use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut hexadecimal = String::new();
    reader.read_line(&mut hexadecimal).unwrap();

    let bits = hexadecimal
        .chars()
        .flat_map(|c| hex_to_bits(c))
        .collect::<String>();

    let (version_sum, _) = calculate(&bits);
    println!("sum version = {}", version_sum);
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

fn calculate(bits: &str) -> (i64, usize) {
    let type_id = u16::from_str_radix(&bits[3..6], 2).unwrap();

    if type_id == 4 {
        let (number, i) = calculate_number(&bits[6..]);
        return (number, 6 + i);
    } else {
        let length_type_id = u16::from_str_radix(&bits[6..7], 2).unwrap();
        let mut numbers = vec![];
        let mut index = 0;

        if length_type_id == 0 {
            let bits_subpackets = usize::from_str_radix(&bits[7..22], 2).unwrap();

            while index < bits_subpackets {
                let (number, j) = calculate(&bits[22 + index..]);
                numbers.push(number);
                index += j;
            }
            index += 22;
        } else {
            let number_packets = usize::from_str_radix(&bits[7..18], 2).unwrap();

            index = 18;
            let mut i = 0;
            while i < number_packets {
                let (number, j) = calculate(&bits[index..]);
                numbers.push(number);
                index += j;
                i += 1;
            }
        }

        let result = match type_id {
            0 => numbers.iter().sum(),
            1 => numbers.iter().product(),
            2 => *numbers.iter().min().unwrap(),
            3 => *numbers.iter().max().unwrap(),
            5 => (numbers[0] > numbers[1]) as i64,
            6 => (numbers[0] < numbers[1]) as i64,
            7 => (numbers[0] == numbers[1]) as i64,
            _ => 0,
        };

        return (result, index);
    }
}

fn calculate_number(bits: &str) -> (i64, usize) {
    let mut number = String::new();
    let mut i = 0;
    let mut final_packet_reached = false;
    while !final_packet_reached {
        final_packet_reached = u16::from_str_radix(&bits[i..i + 1], 2).unwrap() == 0;
        number += &bits[i + 1..i + 5];
        i += 5;
    }

    (i64::from_str_radix(&number, 2).unwrap(), i)
}
