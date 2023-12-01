use common;

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input1 = common::read_input_file("input1.txt");

    let values = get_calibration_values(input1);
    let sum = calibration_sum(values);

    println!("Puzzle Result: {sum}");
}

fn get_calibration_values(input: String) -> Vec<u8> {
    let words = input.split('\n').collect::<Vec<&str>>();
    let mut values = Vec::new();

    for word in words.iter() {
        let mut digits: Vec<u8> = Vec::new();

        let mut last_digit = 0;
        for (index, c) in word.chars().enumerate() {
            if c.is_numeric() {
                digits.push(c.to_digit(10).unwrap() as u8);
                last_digit = index;
            }

            if let Some(digit) = get_word_digit(&word[last_digit..index + 1]) {
                digits.push(digit);
                last_digit = index;
            };
        }

        if digits.len() > 1 {
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            let value = format!("{first}{last}").parse::<u8>().unwrap();
            values.push(value);
        } else {
            if let Some(first) = digits.first() {
                let value = format!("{first}{first}").parse::<u8>().unwrap();
                values.push(value);
            }
        }
    }

    values
}

fn get_word_digit(word: &str) -> Option<u8> {
    for (index, digit_word) in DIGIT_WORDS.iter().enumerate() {
        if word.contains(digit_word) {
            return Some((index + 1) as u8);
        }
    }

    None
}

fn calibration_sum(values: Vec<u8>) -> u32 {
    let mut sum: u32 = 0;

    for value in values {
        sum += value as u32;
    }

    sum
}
