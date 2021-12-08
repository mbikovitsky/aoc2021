use std::{collections::HashMap, str::FromStr};

use anyhow::Result;

use aoc2021::util::input_lines;
use itertools::Itertools;

const SIGNAL_NAMES: &[char; 7] = &['a', 'b', 'c', 'd', 'e', 'f', 'g'];
const SEGMENT_NAMES: &[char; 7] = SIGNAL_NAMES;
const DIGITS: [(&str, u8); 10] = [
    ("abcefg", 0),
    ("cf", 1),
    ("acdeg", 2),
    ("acdfg", 3),
    ("bcdf", 4),
    ("abdfg", 5),
    ("abdefg", 6),
    ("acf", 7),
    ("abcdefg", 8),
    ("abcdfg", 9),
];

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
            .map(|pattern| pattern.to_string())
            .collect();
        let unique_patterns: [_; 10] = unique_patterns.try_into().unwrap();

        let displayed_patterns: Vec<_> = parts[1]
            .split_whitespace()
            .map(|pattern| pattern.to_string())
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

    let display_sum: u32 = notes.iter().map(|data| decode_display(data)).sum();
    dbg!(display_sum);

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

fn decode_display(data: &DisplayData) -> u32 {
    let mappings = solve_segment_mapping(&data.unique_patterns);

    for mapping in mappings {
        let mapped_patterns: Vec<String> = data
            .displayed_patterns
            .iter()
            .map(|pattern| {
                pattern
                    .chars()
                    .map(|char| mapping.get(&char).unwrap())
                    .collect()
            })
            .collect();

        let digits: Option<Vec<u8>> = mapped_patterns
            .iter()
            .map(|pattern| segment_pattern_to_digit(pattern))
            .collect();

        if let Some(digits) = digits {
            let number = digits
                .into_iter()
                .fold(0u32, |acc, digit| acc * 10 + digit as u32);
            return number;
        }
    }

    panic!("No solution found!");
}

fn solve_segment_mapping(unique_patterns: &[String; 10]) -> Vec<HashMap<char, char>> {
    let mut cache = HashMap::new();
    let mut solutions = vec![];
    backtrack(&mut cache, unique_patterns, &mut solutions);
    assert!(!solutions.is_empty());

    solutions
        .into_iter()
        .map(|solution| {
            let result: HashMap<char, char> = solution
                .into_iter()
                .map(|(signal, segment)| (segment, signal))
                .collect();
            assert_eq!(result.len(), SEGMENT_NAMES.len());

            result
        })
        .collect()
}

fn backtrack(
    assignment: &mut HashMap<char, char>,
    unique_patterns: &[String; 10],
    solutions: &mut Vec<HashMap<char, char>>,
) {
    if assignment.len() == SIGNAL_NAMES.len() {
        solutions.push(assignment.clone());
        return;
    }

    let variable = *SIGNAL_NAMES
        .iter()
        .find(|signal| !assignment.contains_key(&signal))
        .unwrap();

    let values = SEGMENT_NAMES
        .iter()
        .filter(|segment| !assignment.values().contains(segment))
        .copied()
        .collect_vec();

    for value in values {
        assignment.insert(variable, value);
        if is_valid_assignment(assignment, unique_patterns) {
            backtrack(assignment, unique_patterns, solutions);
        }
        assignment.remove(&variable);
    }
}

fn is_valid_assignment(assignment: &HashMap<char, char>, unique_patterns: &[String; 10]) -> bool {
    let one = unique_patterns
        .iter()
        .find(|pattern| pattern.len() == 2)
        .unwrap();
    let seven = unique_patterns
        .iter()
        .find(|pattern| pattern.len() == 3)
        .unwrap();
    let four = unique_patterns
        .iter()
        .find(|pattern| pattern.len() == 4)
        .unwrap();

    // Unary constraints
    for (signal, segment) in assignment {
        for pattern in [one, seven, four] {
            match pattern.len() {
                // 1
                2 => {
                    let signals = ['c', 'f'];
                    if signals.contains(signal) && !pattern.contains(*segment) {
                        return false;
                    }
                }

                // 7
                3 => {
                    let signals = ['a', 'c', 'f'];
                    if signals.contains(signal) && !pattern.contains(*segment) {
                        return false;
                    }
                }

                // 4
                4 => {
                    let signals = ['b', 'c', 'd', 'f'];
                    if signals.contains(signal) && !pattern.contains(*segment) {
                        return false;
                    }
                }

                _ => {
                    unreachable!();
                }
            }
        }
    }

    true
}

fn segment_pattern_to_digit(pattern: &str) -> Option<u8> {
    let mut chars: Vec<_> = pattern.chars().collect();
    chars.sort();
    let canonical_pattern: String = chars.into_iter().collect();

    DIGITS
        .iter()
        .find(|(pattern, _digit)| *pattern == canonical_pattern)
        .map(|(_, digit)| *digit)
}

fn parse_input() -> Result<Vec<DisplayData>> {
    input_lines()?.map(|line| Ok(line?.parse()?)).collect()
}
