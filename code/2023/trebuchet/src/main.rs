use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {


    let file = File::open("inputs/trebuchet.txt").expect("could not load file.");
    let reader = BufReader::new(file);

    let total_digit_sum: u32 = reader
        .lines()
        .into_iter()
        .map(|line_result| line_result.expect("could not read line"))
        .map(|line| combine_first_and_last_digit(&line))
        .sum();

    dbg!(total_digit_sum);
}

fn combine_first_and_last_digit(line: &String) -> u32 {
    let digits = [
        "0", "zero",
        "1", "one",
        "2", "two",
        "3", "three",
        "4", "four",
        "5", "five",
        "6", "six",
        "7", "seven",
        "8", "eight",
        "9", "nine",
    ];

    let reverse_digits = [
        "0", "orez",
        "1", "eno",
        "2", "owt",
        "3", "eerht",
        "4", "ruof",
        "5", "evif",
        "6", "xis",
        "7", "neves",
        "8", "thgie",
        "9", "enin",
    ];

    let first_digit = find_first_digit(&line, &digits);
    let last_digit = find_first_digit(&line.chars().rev().collect(), &reverse_digits);
    format!("{}{}", first_digit, last_digit)
        .parse()
        .expect("could not convert string to int")
}

fn find_first_digit(line: &String, digits: &[&str]) -> u32 {
    let digit_to_u32 = HashMap::from([
        ("0", 0u32),
        ("1", 1u32),
        ("2", 2u32),
        ("3", 3u32),
        ("4", 4u32),
        ("5", 5u32),
        ("6", 6u32),
        ("7", 7u32),
        ("8", 8u32),
        ("9", 9u32),
        ("zero", 0u32),
        ("one", 1u32),
        ("two", 2u32),
        ("three", 3u32),
        ("four", 4u32),
        ("five", 5u32),
        ("six", 6u32),
        ("seven", 7u32),
        ("eight", 8u32),
        ("nine", 9u32),
        ("orez", 0u32),
        ("eno", 1u32),
        ("owt", 2u32),
        ("eerht", 3u32),
        ("ruof", 4u32),
        ("evif", 5u32),
        ("xis", 6u32),
        ("neves", 7u32),
        ("thgie", 8u32),
        ("enin", 9u32),
    ]);

    dbg!(line);
    for i in 0..line.len() {
        for digit in digits {
            if line[i..line.len()].starts_with(digit) {
                dbg!(digit);
                match digit_to_u32.get(digit) {
                    Some(u32_digit) => return *u32_digit,
                    None => {eprintln!("Could not find digit {digit}")}
                }
            }
        }
    }
    0
}