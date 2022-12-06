use std::collections::HashSet;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Read};

use anyhow::Result as AnyResult;

use itertools::Itertools;

fn main() -> AnyResult<()> {
    let input = read_to_string("day-06/input.txt")?;

    println!("{}", solve1(&input)?);
    println!("{}", solve2(&input)?);

    Ok(())
}

fn solve1(input: &str) -> AnyResult<i32> {
    for (ix, (c1, c2, c3, c4)) in input.chars().tuple_windows().enumerate() {
        let mut set = HashSet::new();
        set.insert(c1);
        set.insert(c2);
        set.insert(c3);
        set.insert(c4);
        if set.len() == 4 {
            return Ok(ix as i32 + 4);
        }
    }

    Ok(-1)
}

fn solve2(input: &str) -> AnyResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test1() {
        assert_eq!(
            solve1(INPUT).unwrap(),
            7
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve2(INPUT).unwrap(),
            0
        );
    }
}
