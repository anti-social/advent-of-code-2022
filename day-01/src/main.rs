use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    let input = BufReader::new(File::open("day-01/input.txt")?);

    let mut top3_elves_calories = BinaryHeap::new();
    let mut cur_elf_calories = 0u64;
    for line in input.lines() {
        let row = line?;
        if row.is_empty() {
            top3_elves_calories.push(cur_elf_calories);
            cur_elf_calories = 0;
            continue;
        }
        cur_elf_calories += row.parse::<u64>()?;
    }

    let max_calories = top3_elves_calories.peek().unwrap();
    println!("{}", max_calories);

    println!("{}", top3_elves_calories.iter().take(3).sum::<u64>());
    Ok(())
}
