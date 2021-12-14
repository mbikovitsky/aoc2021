use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use aoc2021::util::input_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule {
    source: [char; 2],
    replacement: char,
}

fn main() -> Result<()> {
    let (template, rules) = parse_input()?;

    let current = replace_n(&template, &rules, 10);
    let (most_common, least_common) = most_least_common(&current);
    let part1 = most_common.1 - least_common.1;
    dbg!(part1);

    let current = replace_n(&template, &rules, 40);
    let (most_common, least_common) = most_least_common(&current);
    let part2 = most_common.1 - least_common.1;
    dbg!(part2);

    Ok(())
}

fn replace_once(template: &str, rules: &[Rule]) -> String {
    let mut result = String::new();

    let mut previous_replaced = false;
    for (a, b) in template.chars().tuple_windows() {
        if let Some(rule) = rules.iter().find(|rule| rule.source == [a, b]) {
            if !previous_replaced {
                result.push(a);
            }
            result.push(rule.replacement);
            result.push(b);
            previous_replaced = true;
        } else {
            previous_replaced = false;
        }
    }

    result
}

fn replace_n(template: &str, rules: &[Rule], count: usize) -> String {
    let mut current = template.to_string();
    for _ in 0..count {
        current = replace_once(&current, &rules);
    }
    current
}

fn most_least_common(string: &str) -> ((char, usize), (char, usize)) {
    let occurrences = count_occurrences(string);
    let most_common = occurrences.iter().max_by_key(|(_, count)| *count).unwrap();
    let least_common = occurrences.iter().min_by_key(|(_, count)| *count).unwrap();

    (
        (*most_common.0, *most_common.1),
        (*least_common.0, *least_common.1),
    )
}

fn count_occurrences(string: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for char in string.chars() {
        if let Some(count) = result.get_mut(&char) {
            *count += 1;
        } else {
            result.insert(char, 1);
        }
    }

    return result;
}

fn parse_input() -> Result<(String, Vec<Rule>)> {
    let mut lines = input_lines()?;

    let template = lines.next().context("No template")??;

    if !lines.next().context("Unexpected EOF")??.is_empty() {
        bail!("Missing empty line");
    }

    let mut rules = vec![];
    for line in lines {
        let line = line?;

        let parts = line.split_once("->").context("Invalid rule format")?;

        let source = parts.0.trim();
        if source.chars().count() != 2 {
            bail!("Invalid number of source characters");
        }
        let source = [
            source.chars().nth(0).unwrap(),
            source.chars().nth(1).unwrap(),
        ];

        let replacement = parts.1.trim();
        if replacement.chars().count() != 1 {
            bail!("Invalid number of replacement characters");
        }
        let replacement = replacement.chars().nth(0).unwrap();

        rules.push(Rule {
            source,
            replacement,
        })
    }

    Ok((template, rules))
}
