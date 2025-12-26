use aoc25_rust::run_main;
use std::io;

fn main() -> io::Result<()> {
    run_main(part1, part2)
}

/// Extracts ranges from the first line of input.
/// Each range is represented as a tuple of (start, end).
/// Also extracts the number of digits in the start and end of each range.
fn extract_ranges(
    mut line_reader: impl Iterator<Item = String>,
) -> (Vec<(u64, u64)>, Vec<(u32, u32)>) {
    let mut ranges = Vec::new();
    let mut ranges_digits = Vec::new();
    let range_string = line_reader.next().expect("expected exactly one item.");
    assert!(line_reader.next().is_none(), "expected exactly one item.");
    for part in range_string.split(',') {
        let (start_string, end_string) = part
            .split_once('-')
            .expect("expected exactly two bounds per range");
        let start: u64 = start_string
            .parse()
            .expect("failed to parse range start as integer");
        let end: u64 = end_string
            .parse()
            .expect("failed to parse range start as integer");

        let start_digits = start_string.len() as u32;
        let end_digits = end_string.len() as u32;

        ranges.push((start, end));
        ranges_digits.push((start_digits, end_digits));
    }
    (ranges, ranges_digits)
}

fn part1(mut line_reader: impl Iterator<Item = String>) -> io::Result<()> {
    let (ranges, ranges_digits) = extract_ranges(&mut line_reader);
    assert_eq!(
        ranges.len(),
        ranges_digits.len(),
        "expected same number of ranges and digit counts"
    );
    let mut invalid_id_sum: u64 = 0;
    for (range, range_digits) in ranges.iter().zip(ranges_digits.iter()) {
        // divide into digit ranges (deal with 0-9, 10-99, etc.)
        // for full digit ranges (0-9, 10-99, etc.), we can directly compute the sum of invalid IDs
        // for partial digit ranges (e.g., 23-456), we need to adjust start (23 - 99) and end (100 - 456) accordingly
        for num_digits in range_digits.0..=range_digits.1 {
            if num_digits % 2 != 0 {
                // skip odd digit counts
                continue;
            }
            let half_digits = num_digits / 2;
            let mut start_half: u64 = 10_u64.pow(half_digits - 1);
            let mut end_half: u64 = 10_u64.pow(half_digits) - 1;

            if num_digits == range_digits.0 {
                start_half = range.0 / 10_u64.pow(half_digits);
                let start_bottom_half = range.0 % 10_u64.pow(half_digits);
                if start_half < start_bottom_half {
                    // if start half being repeated is not feasible since start_half < start_bottom_half
                    // e.g., 2300 -> 23|00, need to start from 24
                    start_half += 1;
                }
            }
            if num_digits == range_digits.1 {
                end_half = range.1 / 10_u64.pow(half_digits);
                let end_bottom_half = range.1 % 10_u64.pow(half_digits);
                if end_half > end_bottom_half {
                    // reverse logic here where end_half > end_bottom_half
                    // e.g., 2322 -> 23|22, need to end at 22
                    end_half -= 1;
                }
            }

            if start_half > end_half {
                // this can occur if no valid halves in range
                continue;
            }

            // sum of arithmetic series
            let sum_halves = (end_half - start_half + 1) * (start_half + end_half) / 2;
            invalid_id_sum += sum_halves * (10_u64.pow(half_digits) + 1);
        }
    }
    println!("Part 1: Sum of invalid IDs: {}", invalid_id_sum);
    Ok(())
}

fn is_primitive(s: &str) -> bool {
    let d = s.len();
    for p in 1..d {
        if d % p != 0 {
            continue;
        }
        let mut ok = true;
        for i in p..d {
            if s.as_bytes()[i] != s.as_bytes()[i % p] {
                ok = false;
                break;
            }
        }
        if ok {
            return false; // periodic
        }
    }
    true // primitive
}

fn get_repeat_int(base: u64, base_len: u32, repeat_count: u32) -> u64 {
    let mut result = 0u64;
    let mut factor = 1u64;
    for _ in 0..repeat_count {
        result += base * factor;
        factor *= 10u64.pow(base_len);
    }
    result
}

fn calculate_sum_of_n_digit_base(
    num_digits: u32,
    base_len: u32,
    base_min: u64,
    base_max: u64,
) -> u64 {
    let mut total_sum: u64 = 0;
    for base in base_min..=base_max {
        let base_string = base.to_string();
        if !is_primitive(&base_string) {
            continue;
        }
        let repeated_int = get_repeat_int(base, base_len, num_digits / base_len);
        total_sum += repeated_int;
    }
    total_sum
}

fn calculate_sum(num_digits: u32, start: u64, end: u64) -> u64 {
    let mut total_sum: u64 = 0;
    for base_len in 1..=num_digits / 2 {
        if num_digits % base_len != 0 {
            continue;
        }
        let n_repetitions = num_digits / base_len;
        let mut multiplier = 1u64;
        for _ in 1..n_repetitions {
            multiplier = multiplier * 10u64.pow(base_len) + 1;
        }

        // Compute base_min/base_max to stay within [start, end]
        let base_min = (start + multiplier - 1) / multiplier; // ceil
        let base_max = end / multiplier; // floor

        if base_min > base_max {
            continue;
        }

        total_sum += calculate_sum_of_n_digit_base(num_digits, base_len, base_min, base_max);
    }
    total_sum
}

fn part2(mut line_reader: impl Iterator<Item = String>) -> io::Result<()> {
    let (ranges, ranges_digits) = extract_ranges(&mut line_reader);
    assert_eq!(
        ranges.len(),
        ranges_digits.len(),
        "expected same number of ranges and digit counts"
    );
    let mut invalid_id_sum: u64 = 0;
    for (range, range_digits) in ranges.iter().zip(ranges_digits.iter()) {
        for num_digits in range_digits.0..=range_digits.1 {
            let mut start = 10_u64.pow(num_digits - 1);
            let mut end = 10_u64.pow(num_digits) - 1;
            if num_digits == range_digits.0 {
                start = range.0;
            }
            if num_digits == range_digits.1 {
                end = range.1;
            }
            invalid_id_sum += calculate_sum(num_digits, start, end);
        }
    }
    println!("Part 2: Sum of invalid IDs: {}", invalid_id_sum);
    Ok(())
}
