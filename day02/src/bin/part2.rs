use anyhow::Result;
use std::fs;

enum RPS {
    Rock,
    Paper,
    Scissors,
}
impl RPS {
    fn score(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}
enum RoundResult {
    Win,
    Draw,
    Loss,
}
impl RoundResult {
    fn score(&self) -> i32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }
}

///  Returns the RPS to pick to get the desired result
fn pick(other: &RPS, result: &RoundResult) -> RPS {
    match (other, result) {
        (RPS::Rock, RoundResult::Win) => RPS::Paper,
        (RPS::Rock, RoundResult::Draw) => RPS::Rock,
        (RPS::Rock, RoundResult::Loss) => RPS::Scissors,
        (RPS::Paper, RoundResult::Win) => RPS::Scissors,
        (RPS::Paper, RoundResult::Draw) => RPS::Paper,
        (RPS::Paper, RoundResult::Loss) => RPS::Rock,
        (RPS::Scissors, RoundResult::Win) => RPS::Rock,
        (RPS::Scissors, RoundResult::Draw) => RPS::Scissors,
        (RPS::Scissors, RoundResult::Loss) => RPS::Paper,
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let score = input.lines().fold(0, |acc, round| {
        let parts: Vec<&str> = round.split(' ').collect();
        let a = match parts[1] {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => unreachable!(),
        };
        let b = match parts[0] {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => unreachable!(),
        };
        let p = pick(&b, &a);
        // We already know the round score so use that directly in combination with the pick score
        acc + p.score() + a.score()
    });

    println!("Score: {}", score);
    Ok(())
}
