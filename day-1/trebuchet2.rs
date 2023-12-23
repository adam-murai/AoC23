const DIGITS_ENGLISH: &'static [&'static str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

use std::fs;
use std::convert::TryInto;

fn get_first_digit(s: &str) -> u32 {
    let mut chars = s.char_indices();
    while let Some((i, c)) = chars.next() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
        for (i_d, digit_english) in DIGITS_ENGLISH.iter().enumerate() {
            let last = i + digit_english.len();
            if last < s.len() && s[i .. last] == **digit_english {
                return (i_d + 1).try_into().unwrap()
            }
        }
    }
    panic!("no digit found");
}

fn get_last_digit(s: &str) -> u32 {
    let mut chars = s.char_indices().rev();
    while let Some((i, c)) = chars.next() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
        for (i_d, digit_english) in DIGITS_ENGLISH.iter().enumerate() {
            let last: i32 = i as i32 - digit_english.len() as i32 + 1;
            if last >= 0 && s[last as usize .. i + 1] == **digit_english {
                return (i_d + 1).try_into().unwrap()
            }
        }
    }
    panic!("no digit found");
}

fn decode_string(encoded: &str) -> u32 {
    let tens_digit = get_first_digit(encoded);
    let ones_digit = get_last_digit(encoded);

    10 * tens_digit + ones_digit
}

fn main() {
    println!("Hello world!");
    println!("decoded: {}", decode_string("two1nine"));

    let encoded_lines = fs::read_to_string("./encoded.txt").unwrap();
    let sum = encoded_lines.lines().fold(0_u32,
        | acc: u32, line: &str | acc + decode_string(line)
    );

    println!("sum of decoded: {}", sum);
}