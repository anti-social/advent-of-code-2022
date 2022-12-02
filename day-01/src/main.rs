use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    let input = BufReader::new(File::open("day-01/input.txt")?);

    let mut max_calories = 0u64;
    let mut cur_elf_calories = 0u64;
    for line in input.lines() {
        let row = line?;
        if row.is_empty() {
            if cur_elf_calories > max_calories {
                max_calories = cur_elf_calories;
            }
            cur_elf_calories = 0;
            continue;
        }
        cur_elf_calories += row.parse::<u64>()?;
    }

    println!("{}", max_calories);
    Ok(())
}
