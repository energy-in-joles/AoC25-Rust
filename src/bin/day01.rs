use aoc25_rust::cli_parse;
use aoc25_rust::read_input_lines;
use aoc25_rust::AoCArgs;
use std::io;

const START_POSITION: i32 = 50;
const DIAL_RANGE: i32 = 100;

fn parse_movement(s: &str) -> Option<i32> {
    let mut chars = s.chars();
    let first = chars.next()?; // first character
    let rest = chars.as_str(); // remaining string slice
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
    match args.p {
        Some(1) => part1(&args)?,
        Some(2) => part2(&args)?,
        None => {
            part1(&args)?;
            part2(&args)?;
        }
        Some(_) => println!("Invalid part number. Use 1 or 2."),
    }
    Ok(())
}

fn part1(args: &AoCArgs) -> io::Result<()> {
    let mut position = START_POSITION;
    let mut zero_count = 0;
    for line in read_input_lines(args.s)? {
        let movement = parse_movement(&line).expect("Failed to parse movement command");
        position += movement;
        if position % DIAL_RANGE == 0 {
            zero_count += 1;
        }
    }
    println!("Part 1: Times zero position reached: {}", zero_count);
    Ok(())
}

fn wrap_position(pos: i32) -> i32 {
    (pos % DIAL_RANGE + DIAL_RANGE) % DIAL_RANGE
}

fn count_zero_crossover(current_position: i32, movement: i32) -> i32 {
    let q = movement / DIAL_RANGE;
    let r = movement % DIAL_RANGE;
    if (r != 0 && current_position != 0)
        && (((current_position + r) >= DIAL_RANGE) || ((current_position + r) <= 0))
    {
        return 1 + q.abs();
    }
    q.abs()
}

// fn count_zero_crossover(current: i32, movement: i32) -> i32 {
//     let start_div = current.div_euclid(DIAL_RANGE);
//     let end_div = (current + movement).div_euclid(DIAL_RANGE);
//     (end_div - start_div).abs()
// }

fn part2(args: &AoCArgs) -> io::Result<()> {
    let mut position = START_POSITION;
    let mut crossover_count = 0;
    for line in read_input_lines(args.s)? {
        let movement = parse_movement(&line).expect("Failed to parse movement command");
        crossover_count += count_zero_crossover(position, movement);
        position = wrap_position(position + movement);
    }
    println!(
        "Part 2: Times zero position was crossed over: {}",
        crossover_count
    );
    Ok(())
}
