// Part 1:  https://gist.github.com/uherman/5bf71c373b9ce770e2826b391f7f54b9
// Part 2: https://gist.github.com/uherman/711b06d6b8b376f39f054034a240b661
use std::{ fs, str::Chars };

fn main() {
    let contents: String = fs
        ::read_to_string("src/input.txt")
        .expect("Should have been able to read the file");

    let lines: std::str::Split<'_, &str> = contents.split("\n");
    let mut total = 0;

    for line in lines {
        total += calculate_calibration_value(line).expect(
            "Should have been able to calculate calibration value"
        );
    }

    println!("Total calibration value: {}", total);
}

fn calculate_calibration_value(line: &str) -> Option<u32> {
    let digit_1 = find_first_digit(line.chars())?;
    let digit_2 = find_first_digit(reverse_string(line).chars())?;
    Some(combine_digits(digit_1, digit_2))
}

fn combine_digits(digit_1: u32, digit_2: u32) -> u32 {
    format!("{}{}", digit_1, digit_2).as_str().parse::<u32>().unwrap()
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn find_first_digit(mut chars: Chars<'_>) -> Option<u32> {
    let mut word: String = String::new();

    chars.find_map(|char| {
        word += &char.to_string();
        try_parse_digit(char, &word)
    })
}

fn try_parse_digit(char: char, word: &str) -> Option<u32> {
    char.to_digit(10)
        .or_else(|| find_spelled_digit(&word))
        .or_else(|| find_spelled_digit(&reverse_string(&word)))
}

static DIGIT_MAP: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_spelled_digit(word: &str) -> Option<u32> {
    DIGIT_MAP.iter().find_map(|&(name, value)| {
        if word.contains(name) {
            return Some(value);
        }

        None
    })
}
