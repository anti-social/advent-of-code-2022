use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Read};

use anyhow::Result as AnyResult;

use itertools::Itertools;

fn main() -> AnyResult<()> {
    let input = read_to_string("day-07/input.txt")?;

    println!("{}", solve1(&input)?);
    println!("{}", solve2(&input)?);

    Ok(())
}

fn solve1(input: &str) -> AnyResult<u32> {
    Ok(0)
}

fn solve2(input: &str) -> AnyResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    const INPUT: &str = r#"
    "#;

    #[test]
    fn test1() {
        assert_eq!(
            solve1(INPUT).unwrap(),
            0u32
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve2(INPUT).unwrap(),
            0u32
        );
    }
}
