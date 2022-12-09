use std::{cmp::max, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let max_calories: i32 = input
        .lines()
        .fold((Vec::with_capacity(3), 1), |mut acc, line| {
            if line.is_empty() {
                if acc.0.len() < 3 {
                    acc.0.push(acc.1);
                } else {
                    if acc.1 > acc.0[2] {
                        acc.0[2] = acc.1;
                        acc.0.sort();
                        acc.0.reverse();
                    }
                }
                acc.1 = 0;
            } else {
                let calories: i32 = line.parse().unwrap();
                acc.1 += calories;
            }
            acc
        })
        .0
        .iter()
        .sum();
    println!("max calories: {}", max_calories);
    Ok(())
}
