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
    let total_score = input.lines()
        .map(|l| l.unwrap().trim().to_string())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (section1, section2) = l.split_once(',').unwrap();
            (parse_section(section1), parse_section(section2))
        })
        .filter(|sections| {
            sections.0.0 >= sections.1.0 && sections.0.1 <= sections.1.1 ||
            sections.1.0 >= sections.0.0 && sections.1.1 <= sections.0.1
        })
        .count();

    Ok(total_score as u32)
}

fn parse_section(s: &str) -> (u32, u32) {
    let (section_start, section_end) = s.split_once('-').unwrap();
    (section_start.parse().unwrap(), section_end.parse().unwrap())
}

fn solve2(input: impl BufRead) -> AnyResult<u32> {
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

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    const INPUT: &str = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "#;

    #[test]
    fn test1() {
        assert_eq!(
            solve1(INPUT.as_bytes()).unwrap(),
            2u32
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
