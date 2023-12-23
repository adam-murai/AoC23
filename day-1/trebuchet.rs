use std::fs;

fn get_first_digit(chars: &mut dyn Iterator<Item = char>) -> u32 {
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
    }
    panic!("no digit found");
}

fn decode_string(encoded: &str) -> u32 {
    let mut forward_chars = encoded.chars();
    let mut rev_chars = encoded.chars().rev();
    let tens_digit = get_first_digit(&mut forward_chars);
    let ones_digit = get_first_digit(&mut rev_chars);

    10 * tens_digit + ones_digit
}

fn main() {
    println!("Hello world!");
    println!("decoded: {}", decode_string("1six7396484"));

    let encoded_lines = fs::read_to_string("./encoded.txt").unwrap();
    let sum = encoded_lines.lines().fold(0_u32,
        | acc: u32, line: &str | acc + decode_string(line)
    );

    println!("sum of decoded: {}", sum);
}