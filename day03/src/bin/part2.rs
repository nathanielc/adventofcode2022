use itertools::Itertools;
use std::{collections::HashSet, fs};

use anyhow::Result;

const LOWER_CHAR_OFFSET: u32 = 'a' as u32 - 1;
const UPPER_CHAR_OFFSET: u32 = 'A' as u32 - 27;

fn priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        (item as u32) - UPPER_CHAR_OFFSET
    } else {
        (item as u32) - LOWER_CHAR_OFFSET
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let priorities = input
        .lines()
        .into_iter()
        .chunks(3)
        .into_iter()
        .fold(0_u32, |acc, lines| {
            let priority: HashSet<u32> = lines
                .map(|line| line.chars().map(priority).collect::<HashSet<u32>>())
                .fold(None, |acc: Option<HashSet<u32>>, set| {
                    if let Some(acc) = acc {
                        Some(acc.intersection(&set).map(|i| *i).collect())
                    } else {
                        Some(set)
                    }
                })
                .expect("must have at least one common item");
            assert_eq!(1, priority.len());
            acc + priority.iter().next().unwrap()
        });
    println!("Priorities: {}", priorities);
    Ok(())
}
