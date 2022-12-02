use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{bail, Result as AnyResult};

fn main() -> AnyResult<()> {
    let input = BufReader::new(File::open("day-02/input.txt")?);

    let score = input.lines()
        .map(|line| {
            let row = line.unwrap();
            let mut encoded_shapes = row.split_whitespace();
            let their_shape = encoded_shapes.next().unwrap().parse::<Shape>().unwrap();
            let mine_shape = encoded_shapes.next().unwrap().parse::<Shape>().unwrap();
            mine_shape.score(&their_shape)
        })
        .sum::<u64>();

    println!("{}", score);

    Ok(())
}

enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn score(&self, their: &Shape) -> u64 {
        use Shape::*;

        let shape_score = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let round_score = match (self, their) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => 6,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        };
        return shape_score + round_score;
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => bail!("Unknown shape: {}", s),
        })
    }
}