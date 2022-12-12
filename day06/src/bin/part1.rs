use anyhow::Result;
use std::{fs, ops::Range};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut set: Vec<char> = Vec::with_capacity(4);
    let mut idx: usize = 0;
    for (i, c) in input.chars().enumerate() {
        if set.len() < 4 {
            set.push(c)
        } else {
            set.drain(0..1);
            set.push(c);
            if is_sop(&set) {
                idx = i+1;
                break;
            }
        };
    }

    println!("idx: {}", idx);
    Ok(())
}

fn is_sop(set: &[char]) -> bool {
    for (i, c) in set.iter().enumerate() {
        for k in &set[i + 1..] {
            if c == k {
                return false;
            }
        }
    }
    return true;
}
