use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use anyhow::Result as AnyResult;

use itertools::Itertools;

fn main() -> AnyResult<()> {
    let mut input = vec!();
    BufReader::new(File::open("day-03/input.txt")?).read_to_end(&mut input)?;

    println!("{}", solve1(&input[..])?);
    println!("{}", solve2(&input[..])?);

    Ok(())
}

fn solve1(input: impl BufRead) -> AnyResult<u64> {
    let mut total_score = 0u64;
    for line in input.lines() {
        let _row = line?;
        let row = _row.trim();
        if row.is_empty() {
            continue;
        }

        let (compartment1, compartment2) = row.split_at(row.len() / 2);
        assert_eq!(compartment1.len(), compartment2.len());
        let compartment1_items = compartment1.chars().collect::<HashSet<_>>();
        let compartment2_items = compartment2.chars().collect::<HashSet<_>>();
        let mut common_items = compartment1_items.intersection(&compartment2_items);

        let the_item = common_items.next().unwrap();
        total_score += score_item(*the_item) as u64;
    }

    Ok(total_score)
}

fn solve2(input: impl BufRead) -> AnyResult<u32> {
    let total_score = input.lines()
        .map(|l| l.unwrap().trim().to_string())
        .filter(|l| !l.is_empty())
        .chunks(3)
        .into_iter()
        .fold(0, |total_score, group| {
            let common_items = group
                .map(|l| l.chars().collect::<HashSet<_>>())
                .reduce(|common_items, items| common_items.intersection(&items).map(|i| *i).collect())
                .unwrap();
            assert_eq!(common_items.len(), 1);
            total_score + common_items.iter().map(|i| score_item(*i)).sum::<u32>()
        });
        
    Ok(total_score)
}

fn score_item(item: char) -> u32 {
    let a_small_code =  'a' as u32;
    let a_big_code =  'A' as u32;
    let item_code = item as u32;
    if item_code >= a_small_code {
        item_code - a_small_code + 1
    } else {
        item_code - a_big_code + 27
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve1, solve2};

    const INPUT: &str = r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn test1() {
        assert_eq!(
            solve1(INPUT.as_bytes()).unwrap(),
            157u64
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve2(INPUT.as_bytes()).unwrap(),
            70u64
        );
    }
}
