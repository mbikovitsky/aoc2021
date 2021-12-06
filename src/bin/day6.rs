use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Result;

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let fish = parse_input()?;

    let demo_population = simulate_lifetime(&[3, 4, 3, 1, 2], 80);
    dbg!(demo_population);

    let population = simulate_lifetime(&fish, 80);
    dbg!(population);

    Ok(())
}

fn simulate_lifetime(fish: &[u8], days: u32) -> usize {
    fish.iter()
        .map(|fish| population_by_day(*fish, days).last().unwrap().1)
        .sum()
}

fn population_by_day(initial_counter: u8, days: u32) -> Vec<(u32, usize)> {
    let mut population = vec![];

    let mut heap = BinaryHeap::from([Reverse(initial_counter)]);

    let mut passed_days: u32 = 0;
    loop {
        population.push((passed_days, heap.len()));

        let days_to_skip = heap.peek().unwrap().0;
        if passed_days.saturating_add(days_to_skip.into()) >= days {
            return population;
        }

        let current_state = heap.into_vec();
        heap = BinaryHeap::with_capacity(current_state.len());
        for Reverse(counter) in current_state {
            if counter == days_to_skip {
                heap.push(Reverse(6));
                heap.push(Reverse(8));
            } else {
                heap.push(Reverse(counter - days_to_skip - 1));
            }
        }

        passed_days += days_to_skip as u32 + 1;
    }
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
