use anyhow::Result;

use aoc2021::util::input_lines;

const PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

const SCORES: [(char, u32); 4] = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];

fn main() -> Result<()> {
    let lines = parse_input()?;

    let score: u32 = lines
        .iter()
        .filter_map(|line| {
            if let Some(first_invalid) = validate_line(line) {
                Some(score(first_invalid.1).unwrap())
            } else {
                None
            }
        })
        .sum();
    dbg!(score);

    Ok(())
}

fn validate_line(line: &str) -> Option<(usize, char)> {
    let mut stack = vec![];

    for (index, char) in line.char_indices() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.pop() {
                    if opening_char(char).unwrap() != last {
                        return Some((index, char));
                    }
                } else {
                    return Some((index, char));
                }
            }
            _ => {}
        }
    }

    None
}

fn opening_char(closing_char: char) -> Option<char> {
    PAIRS
        .iter()
        .find(|(_, close)| *close == closing_char)
        .map(|(open, _)| *open)
}

fn score(c: char) -> Option<u32> {
    SCORES
        .iter()
        .find(|(candidate, _)| *candidate == c)
        .map(|(_, score)| *score)
}

fn parse_input() -> Result<Vec<String>> {
    input_lines()?.collect()
}
