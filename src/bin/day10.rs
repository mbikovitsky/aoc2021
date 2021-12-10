use anyhow::Result;

use aoc2021::util::input_lines;

const PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

const ILLEGAL_CHAR_SCORES: [(char, u32); 4] = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];

const MISSING_CHAR_SCORES: [(char, u32); 4] = [(')', 1), (']', 2), ('}', 3), ('>', 4)];

#[derive(Debug, Clone, PartialEq, Eq)]
enum LineCondition {
    Valid,
    Invalid { index: usize, char: char },
    Incomplete { missing: String },
}

fn main() -> Result<()> {
    let lines = parse_input()?;

    let score: u32 = lines
        .iter()
        .filter_map(|line| {
            if let LineCondition::Invalid { char, .. } = validate_line(line) {
                Some(illegal_char_score(char).unwrap())
            } else {
                None
            }
        })
        .sum();
    dbg!(score);

    let mut completion_scores: Vec<_> = lines
        .iter()
        .filter_map(|line| match validate_line(line) {
            LineCondition::Incomplete { missing } => Some(completion_score(&missing)),
            _ => None,
        })
        .collect();
    let middle_index = (completion_scores.len() - 1) / 2;
    let (_, &mut winner, _) = completion_scores.select_nth_unstable(middle_index);
    dbg!(winner);

    Ok(())
}

fn validate_line(line: &str) -> LineCondition {
    let mut stack = vec![];

    for (index, char) in line.char_indices() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.pop() {
                    if opening_char(char).unwrap() != last {
                        return LineCondition::Invalid { index, char };
                    }
                } else {
                    return LineCondition::Invalid { index, char };
                }
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        return LineCondition::Valid;
    }

    LineCondition::Incomplete {
        missing: stack
            .into_iter()
            .rev()
            .map(closing_char)
            .collect::<Option<String>>()
            .unwrap(),
    }
}

fn opening_char(closing_char: char) -> Option<char> {
    PAIRS
        .iter()
        .find(|(_, close)| *close == closing_char)
        .map(|(open, _)| *open)
}

fn closing_char(opening_char: char) -> Option<char> {
    PAIRS
        .iter()
        .find(|(open, _)| *open == opening_char)
        .map(|(_, close)| *close)
}

fn illegal_char_score(c: char) -> Option<u32> {
    ILLEGAL_CHAR_SCORES
        .iter()
        .find(|(candidate, _)| *candidate == c)
        .map(|(_, score)| *score)
}

fn completion_score(chars: &str) -> u64 {
    chars
        .chars()
        .filter_map(missing_char_score)
        .fold(0, |acc, score| acc * 5 + score as u64)
}

fn missing_char_score(c: char) -> Option<u32> {
    MISSING_CHAR_SCORES
        .iter()
        .find(|(candidate, _)| *candidate == c)
        .map(|(_, score)| *score)
}

fn parse_input() -> Result<Vec<String>> {
    input_lines()?.collect()
}
