use std::fs;

// struct Round {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// const MAX_ROUND = Round {
//     red: 12,
//     green: 13,
//     blue: 14,
// }

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn is_within_max(n: u32, color: char) -> bool {
    match color {
        'r' => n <= MAX_RED,
        'g' => n <= MAX_GREEN,
        'b' => n <= MAX_BLUE,
        _ => panic!("bruuuuuuh"),
    }
}

fn game_if_valid(game_string: &str) -> u32 {
    let mut parts = game_string.split_ascii_whitespace();

    if let None = parts.next() {
        panic!("bruh");
    }

    let game_number_string = parts.next().unwrap();
    let game_number: u32 = game_number_string[0 .. game_number_string.len() - 1].parse().unwrap();

    while let Some(n_string) = parts.next() {
        let n: u32 = n_string.parse().unwrap();
        let color_string = parts.next().unwrap();
        let color_key_char = color_string.bytes().next().unwrap() as char;
        if !is_within_max(n, color_key_char) {
            return 0;
        }
    }

    game_number
}

fn main() {
    let games = fs::read_to_string("./game-log.txt").unwrap();

    let sum = games.lines().fold(0_u32,
        | acc: u32, line: &str | acc + game_if_valid(line)
    );

    println!("sum of invalid games: {}", sum);

}