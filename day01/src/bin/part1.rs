use std::{cmp::max, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let max_calories = input.lines().fold((0, 1), |mut acc, line| {
        if line.is_empty() {
            acc.0 = max(acc.0, acc.1);
            acc.1 = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            acc.1 += calories;
        }
        acc
    });
    println!("max calories: {}", max_calories.0);
    Ok(())
}
