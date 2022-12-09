use anyhow::Result;
use std::{fs, ops::Range};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    // Fold all lines into a count
    let count = input.lines().fold(0, |acc, line| {
        // Parse each line into the ranges
        let ranges: Vec<Range<i32>> = line
            .split(',')
            .map(|part| {
                let range: Vec<&str> = part.split('-').collect();
                assert_eq!(2, range.len());
                Range {
                    start: range[0].parse::<i32>().unwrap(),
                    // Range end is exclusive so bump it
                    end: range[1].parse::<i32>().unwrap() + 1,
                }
            })
            .collect();
        assert_eq!(2, ranges.len());
        // Compare range overlap
        if ranges[0].contains(&ranges[1].start)
            || ranges[0].contains(&(ranges[1].end - 1))
            || ranges[1].contains(&ranges[0].start)
            || ranges[1].contains(&(ranges[0].end - 1))
        {
            acc + 1
        } else {
            acc
        }
    });
    println!("Pairs: {}", count);
    Ok(())
}
