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

fn calculate_population(fish: &[u8], days: u32) -> usize {
    let mut cache = HashMap::new();
    fish.iter()
        .map(|fish| calculate_population_from_one(*fish, days, &mut cache))
        .sum()
}

fn calculate_population_from_one(
    initial_counter: u8,
    days: u32,
    cache: &mut HashMap<(u8, u32), usize>,
) -> usize {
    if let Some(population) = cache.get(&(initial_counter, days)) {
        return *population;
    }

    if days <= initial_counter.into() {
        cache.insert((initial_counter, days), 1);
        return 1;
    }

    let remainder = days - initial_counter as u32 - 1;

    let pop_from_original_fish = calculate_population_from_one(6, remainder, cache);
    let pop_from_new_fish = calculate_population_from_one(8, remainder, cache);
    let total = pop_from_original_fish + pop_from_new_fish;

    cache.insert((initial_counter, days), total);
    total
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
