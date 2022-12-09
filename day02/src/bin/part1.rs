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

/// Returns the score for player a
fn scoreA(a: RPS, b: RPS) -> i32 {
    let r = match (&a, &b) {
        (RPS::Rock, RPS::Rock) => RoundResult::Draw,
        (RPS::Rock, RPS::Paper) => RoundResult::Loss,
        (RPS::Rock, RPS::Scissors) => RoundResult::Win,
        (RPS::Paper, RPS::Rock) => RoundResult::Win,
        (RPS::Paper, RPS::Paper) => RoundResult::Draw,
        (RPS::Paper, RPS::Scissors) => RoundResult::Loss,
        (RPS::Scissors, RPS::Rock) => RoundResult::Loss,
        (RPS::Scissors, RPS::Paper) => RoundResult::Win,
        (RPS::Scissors, RPS::Scissors) => RoundResult::Draw,
    };
    a.score() + r.score()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let score = input.lines().fold(0, |acc, round| {
        let parts: Vec<&str> = round.split(' ').collect();
        let a = match parts[1] {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => unreachable!(),
        };
        let b = match parts[0] {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => unreachable!(),
        };
        acc + scoreA(a, b)
    });

    println!("Score: {}", score);
    Ok(())
}
