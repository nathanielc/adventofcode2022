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
    let priorities = input.lines().fold(0_u32, |acc, line| {
        let count = line.chars().count();
        let a: HashSet<u32> = line.chars().take(count / 2).map(priority).collect();
        let b: HashSet<u32> = line.chars().skip(count / 2).map(priority).collect();
        let i: Vec<u32> = a.intersection(&b).map(|i| *i).collect();
        assert_eq!(1, i.len());
        acc + i[0]
    });
    println!("Priorities: {}", priorities);
    Ok(())
}
