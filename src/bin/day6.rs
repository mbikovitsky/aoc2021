use std::collections::HashMap;

use anyhow::Result;

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let fish = parse_input()?;

    let demo_population = calculate_population(&[3, 4, 3, 1, 2], 80);
    dbg!(demo_population);

    let population = calculate_population(&fish, 80);
    dbg!(population);

    let demo_population_long = calculate_population(&[3, 4, 3, 1, 2], 256);
    dbg!(demo_population_long);

    let population_long = calculate_population(&fish, 256);
    dbg!(population_long);

    Ok(())
}

fn calculate_population(fish: &[u8], days: u32) -> u64 {
    let mut cache = vec![0; days.try_into().unwrap()];

    fn lookup(cache: &[u64], fish: u8, days: u32) -> u64 {
        let fish: u32 = fish.into();

        if days <= fish.into() {
            return 1;
        }

        let index: usize = (days - fish - 1).try_into().unwrap();
        cache[index]
    }

    for day in 1..=days {
        let index: usize = (day - 1).try_into().unwrap();
        cache[index] = lookup(&cache, 6, day - 1) + lookup(&cache, 8, day - 1);
    }

    fish.iter().map(|fish| lookup(&cache, *fish, days)).sum()
}

fn parse_input() -> Result<Vec<u8>> {
    let lines: Result<Vec<_>> = input_lines()?.collect();
    let lines = lines?;
    assert_eq!(lines.len(), 1);

    let counters: Result<Vec<_>> = lines[0]
        .split(',')
        .map(|counter| Ok(counter.parse()?))
        .collect();
    let counters = counters?;

    Ok(counters)
}
