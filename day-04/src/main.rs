use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use anyhow::Result as AnyResult;

use itertools::Itertools;

fn main() -> AnyResult<()> {
    let mut input = vec!();
    BufReader::new(File::open("day-04/input.txt")?).read_to_end(&mut input)?;

    println!("{}", solve1(&input[..])?);
    println!("{}", solve2(&input[..])?);

    Ok(())
}

fn solve1(input: impl BufRead) -> AnyResult<u32> {
    let mut total_score = 0u32;
    for line in input.lines() {
        let _row = line?;
        let row = _row.trim();
        if row.is_empty() {
            continue;
        }
    }

    Ok(total_score)
}

fn solve2(input: impl BufRead) -> AnyResult<u32> {
    let total_score = input.lines()
        .map(|l| l.unwrap().trim().to_string())
        .filter(|l| !l.is_empty())
        .map(|l| 0)
        .sum();
        
    Ok(total_score)
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    const INPUT: &str = r#"

    "#;

    #[test]
    fn test1() {
        assert_eq!(
            solve1(INPUT.as_bytes()).unwrap(),
            0u32
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve2(INPUT.as_bytes()).unwrap(),
            0u32
        );
    }
}
