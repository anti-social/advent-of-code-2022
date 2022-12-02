use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{bail, Result as AnyResult};

fn main() -> AnyResult<()> {
    let input = BufReader::new(File::open("day-02/input.txt")?);

    let mut score1 = 0u64;
    let mut score2 = 0u64;
    for line in input.lines() {
        let row = line?;
        let mut encoded_shapes = row.split_whitespace();
        let their_shape = encoded_shapes.next().unwrap().parse::<Shape>()?;
        let encoded = encoded_shapes.next().unwrap();
        let mine_shape = encoded.parse::<Shape>()?;
        score1 += mine_shape.score(&their_shape);

        let mine_shape = match encoded.parse::<Round>()? {
            Round::Lose => their_shape.losing_opponent(),
            Round::Draw => their_shape.draw_opponent(),
            Round::Win => their_shape.winning_opponent(),
        };
        score2 += mine_shape.score(&their_shape);
    }

    println!("{}", score1);
    println!("{}", score2);

    Ok(())
}

#[derive(Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    const BEATS: &'static [(Shape, Shape)] = &[
        (Shape::Rock, Shape::Scissors),
        (Shape::Paper, Shape::Rock),
        (Shape::Scissors, Shape::Paper),
    ];

    fn winning_opponent(&self) -> Self {
        for (oppo, mine) in Self::BEATS {
            if mine == self {
                return oppo.clone();
            }
        }
        unreachable!()
    }

    fn draw_opponent(&self) -> Self {
        self.clone()
    }

    fn losing_opponent(&self) -> Self {
        for (mine, oppo) in Self::BEATS {
            if mine == self {
                return oppo.clone();
            }
        }
        unreachable!()
    }

    fn score(&self, their: &Shape) -> u64 {
        use Shape::*;

        let shape_score = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let round_score = if self.winning_opponent() == *their {
            0
        } else if self.losing_opponent() == *their {
            6
        } else {
            3
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

enum Round {
    Lose,
    Draw,
    Win,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Round::Lose,
            "Y" => Round::Draw,
            "Z" => Round::Win,
            _ => bail!("Unknown round result: {}", s),
        })
    }
}
