use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    let input = BufReader::new(File::open("day-03/input.txt")?);

    println!("{}", solve1(input)?);

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
        let a_small_code =  'a' as u32;
        let a_big_code =  'A' as u32;
        let item_code = *the_item as u32;
        let item_score = if item_code >= a_small_code {
            item_code - a_small_code + 1
        } else {
            item_code - a_big_code + 27
        };

        total_score += item_score as u64;
    }

    Ok(total_score)
}

#[cfg(test)]
mod tests {
    use crate::solve1;

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
}
