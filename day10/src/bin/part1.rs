use anyhow::Result;
use std::fs;

enum Instruction {
    Noop,
    AddX(i32),
}
fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| match &line[0..4] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(line[5..].parse().unwrap()),
            _ => unreachable!(),
        })
        .collect();
    let mut x = 1_i32;
    let mut cycle = 1;
    let mut sum = 0;
    for inst in instructions {
        match inst {
            Instruction::Noop => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 && cycle <= 220 {
                    sum += cycle * x;
                }
            }
            Instruction::AddX(op) => {
                if (cycle + 1 - 20) % 40 == 0 && cycle + 1 <= 220 {
                    sum += (cycle + 1) * x;
                }
                x += op;
                cycle += 2;
                if (cycle - 20) % 40 == 0 && cycle <= 220 {
                    sum += cycle * x;
                }
            }
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}
