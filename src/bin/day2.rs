use anyhow::{bail, Context, Result};

use aoc2021::util::input_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn main() -> Result<()> {
    let directions = parse_input()?;

    let mut depth = 0u32;
    let mut horizontal_pos = 0u32;
    for direction in &directions {
        match direction {
            Direction::Forward(x) => horizontal_pos += x,
            Direction::Down(x) => depth += x,
            Direction::Up(x) => depth -= x,
        }
    }
    dbg!(depth * horizontal_pos);

    let mut depth = 0u32;
    let mut horizontal_pos = 0u32;
    let mut aim = 0u32;
    for direction in &directions {
        match direction {
            Direction::Down(x) => aim += x,
            Direction::Up(x) => aim -= x,
            Direction::Forward(x) => {
                horizontal_pos += x;
                depth += aim * x;
            }
        }
    }
    dbg!(depth * horizontal_pos);

    Ok(())
}

fn parse_input() -> Result<Vec<Direction>> {
    let directions: Result<Vec<_>> = input_lines()?
        .map(|line| {
            let line = line?;

            let mut parts = line.split_whitespace();
            let direction = parts
                .next()
                .with_context(|| format!("No direction in line '{}'", line))?;
            let count: u32 = parts
                .next()
                .with_context(|| format!("No count in line '{}'", line))?
                .parse()
                .with_context(|| format!("Failed parsing count in line '{}'", line))?;

            let direction = match direction {
                "forward" => Direction::Forward(count),
                "down" => Direction::Down(count),
                "up" => Direction::Up(count),
                _ => bail!("Invalid direction '{}'", direction),
            };

            Ok(direction)
        })
        .collect();

    directions
}
