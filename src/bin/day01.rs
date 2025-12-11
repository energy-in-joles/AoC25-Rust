use std::io;
use aoc25_rust::read_input_lines;
use aoc25_rust::cli_parse;

const START_POSITION: i32 = 50;
const DIAL_RANGE: i32 = 100;

fn parse_movement(s: &str) -> Option<i32> {
    let mut chars = s.chars();
    let first = chars.next()?;        // first character
    let rest = chars.as_str();        // remaining string slice
    let steps = rest.parse::<i32>().ok()?; // returns None if parsing fails
    if first == 'R' {
        Some(steps)
    } else if first == 'L' {
        Some(-steps)
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    let args = cli_parse();
    let mut position = START_POSITION;
    let mut zero_count = 0;
    for line in read_input_lines(args.s)? {
        let movement = parse_movement(&line)
            .expect("Failed to parse movement command");
        position += movement;
        if position % DIAL_RANGE == 0 {
            zero_count += 1;
        }
    }
    println!("Times zero position reached: {}", zero_count);
    Ok(())
}