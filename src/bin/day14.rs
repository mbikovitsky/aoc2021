use std::{collections::HashMap, hash::Hash};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use num::{CheckedAdd, Unsigned};

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let (template, rules) = parse_input()?;

    let part1 = solve(&template, &rules, 10);
    dbg!(part1);

    let part2 = solve(&template, &rules, 40);
    dbg!(part2);

    Ok(())
}

fn solve(template: &str, rules: &HashMap<(char, char), char>, iterations: u32) -> u64 {
    let counters = expand_template(&template, &rules, iterations);

    let max = *counters.values().max().unwrap();
    let min = *counters.values().min().unwrap();

    return max - min;
}

fn expand_template(
    template: &str,
    rules: &HashMap<(char, char), char>,
    iterations: u32,
) -> HashMap<char, u64> {
    let counters = expand_rules(rules, iterations);

    let mut result = HashMap::new();

    for (index, (a, b)) in template.chars().tuple_windows().enumerate() {
        result = add_counter_maps(&result, counters.get(&(a, b)).unwrap());
        if index != 0 {
            *result.get_mut(&a).unwrap() -= 1;
        }
    }

    result
}

fn expand_rules(
    rules: &HashMap<(char, char), char>,
    iterations: u32,
) -> HashMap<(char, char), HashMap<char, u64>> {
    let mut current_iteration: HashMap<(char, char), HashMap<char, u64>> = rules
        .keys()
        .copied()
        .map(|source| {
            (
                source,
                if source.0 == source.1 {
                    HashMap::from([(source.0, 2)])
                } else {
                    HashMap::from([(source.0, 1), (source.1, 1)])
                },
            )
        })
        .collect();

    for _ in 0..iterations {
        let next_iteration = rules
            .iter()
            .map(|(&source, &inserted)| {
                let left = current_iteration.get(&(source.0, inserted)).unwrap();
                let right = current_iteration.get(&(inserted, source.1)).unwrap();

                let mut result = add_counter_maps(left, right);
                *result.get_mut(&inserted).unwrap() -= 1;

                (source, result)
            })
            .collect();

        current_iteration = next_iteration;
    }

    current_iteration
}

fn add_counter_maps<K, V>(first: &HashMap<K, V>, second: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Unsigned + CheckedAdd + Copy,
{
    let mut result = first.clone();

    for (key, second_counter) in second.iter() {
        if let Some(first_counter) = result.get_mut(key) {
            *first_counter = first_counter.checked_add(second_counter).unwrap();
        } else {
            result.insert(key.clone(), *second_counter);
        }
    }

    result
}

fn parse_input() -> Result<(String, HashMap<(char, char), char>)> {
    let mut lines = input_lines()?;

    let template = lines.next().context("No template")??;

    if !lines.next().context("Unexpected EOF")??.is_empty() {
        bail!("Missing empty line");
    }

    let mut rules = HashMap::new();
    for line in lines {
        let line = line?;

        let parts = line.split_once("->").context("Invalid rule format")?;

        let source = parts.0.trim();
        let source = source
            .chars()
            .collect_tuple()
            .context("Invalid number of source characters")?;

        let inserted = parts.1.trim();
        if inserted.chars().count() != 1 {
            bail!("Invalid number of replacement characters");
        }
        let inserted = inserted.chars().nth(0).unwrap();

        rules.insert(source, inserted);
    }

    Ok((template, rules))
}
