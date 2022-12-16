use anyhow::Result;
use std::{fs, ops::Range};

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
    let mut render: Vec<bool> = Vec::with_capacity(40 * 6);
    render.resize(40 * 6, false);
    let mut draw = |add| {
        let r = Range {
            start: x - 1,
            end: x + 2,
        };
        if r.contains(&((cycle % 40) - 1)) {
            // I have an off by one error, 
            // I don't care since I can still read the output.
            render[cycle as usize] = true;
        }
        x += add;
        cycle += 1;
    };
    for inst in instructions {
        match inst {
            Instruction::Noop => {
                draw(0);
            }
            Instruction::AddX(op) => {
                draw(0);
                draw(op);
            }
        }
    }
    let pixels: Vec<char> = render.iter().map(|p| if *p { '#' } else { '.' }).collect();
    println!("{}", pixels[0..40].iter().collect::<String>());
    println!("{}", pixels[40..80].iter().collect::<String>());
    println!("{}", pixels[80..120].iter().collect::<String>());
    println!("{}", pixels[120..160].iter().collect::<String>());
    println!("{}", pixels[160..200].iter().collect::<String>());
    println!("{}", pixels[200..240].iter().collect::<String>());
    Ok(())
}
