use std::str::FromStr;

use anyhow::Result;

use aoc2021::util::input_lines;

struct DisplayData {
    unique_patterns: [String; 10],
    displayed_patterns: [String; 4],
}

impl FromStr for DisplayData {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = string.split('|').collect();
        let parts: [_; 2] = parts.try_into().unwrap();

        let unique_patterns: Vec<_> = parts[0]
            .split_whitespace()
            .map(|string| string.to_string())
            .collect();
        let unique_patterns: [_; 10] = unique_patterns.try_into().unwrap();

        let displayed_patterns: Vec<_> = parts[1]
            .split_whitespace()
            .map(|string| string.to_string())
            .collect();
        let displayed_patterns: [_; 4] = displayed_patterns.try_into().unwrap();

        Ok(Self {
            unique_patterns,
            displayed_patterns,
        })
    }
}

fn main() -> Result<()> {
    let notes = parse_input()?;

    let digits_with_unique_amount_of_segments = count_digits_with_unique_amount_of_segments(&notes);
    dbg!(digits_with_unique_amount_of_segments);

    Ok(())
}

fn count_digits_with_unique_amount_of_segments(notes: &[DisplayData]) -> u32 {
    notes
        .iter()
        .map(|data| {
            data.displayed_patterns
                .iter()
                .filter(|pattern| match pattern.len() {
                    2 | 4 | 3 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>()
        .try_into()
        .unwrap()
}

fn parse_input() -> Result<Vec<DisplayData>> {
    input_lines()?.map(|line| Ok(line?.parse()?)).collect()
}
