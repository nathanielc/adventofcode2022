use anyhow::Result;
use std::{collections::HashMap, fs};

use day05::{parser, Term};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut ast = parser::FileParser::new()
        .parse(&input)
        .map_err(|err| err.map_token(|tok| tok.to_string()))?;
    let mut pile: HashMap<usize, Vec<char>> = HashMap::with_capacity(9);
    ast.pile.rows.reverse();
    for row in ast.pile.rows.iter().skip(1) {
        for (i, c) in row.terms.iter().enumerate() {
            match c {
                Term::NullCrate => {
                    //ignore null crates
                }
                Term::Crate(c) => {
                    // Use one based labels
                    let entry = pile.entry(i + 1).or_insert(Vec::new());
                    entry.push(*c);
                }
                Term::Label(_) => panic!("found extra label"),
            };
        }
    }
    println!("pile: {:?}", pile);
    let mut crates = Vec::with_capacity(9);
    for mv in ast.commands {
        println!("move {:?}", mv);
        for _ in 0..mv.count {
            pile.entry(mv.from)
                .and_modify(|stack| crates.push(stack.pop().unwrap()));
        }
        pile.entry(mv.to)
            .and_modify(|stack| stack.append(&mut crates));
        println!("pile: {:?}", pile);
    }
    for i in 1..10 {
        let stack = pile.get(&i).unwrap();
        crates.push(stack[stack.len()-1]);
    }
    println!("crates: {:?}", crates);
    Ok(())
}
