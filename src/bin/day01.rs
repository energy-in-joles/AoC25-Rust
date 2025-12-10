use std::io;
use aoc25_rust::read_input_lines;

const START_POSITION: i32 = 50;

fn main() -> io::Result<()> {
    for line in read_input_lines()? {
        println!("{}", line);
    }
    Ok(())
}