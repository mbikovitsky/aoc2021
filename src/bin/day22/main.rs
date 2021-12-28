mod csg;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::util::input_lines;

use csg::{AABox, CSGObject, Nothing};

#[derive(Debug, Clone)]
enum Instruction {
    On(AABox),
    Off(AABox),
}

fn main() -> Result<()> {
    let instructions = parse_input()?;

    let result = instructions
        .into_iter()
        .filter(|instruction| match instruction {
            Instruction::On(aabox) | Instruction::Off(aabox) => {
                aabox.x.start >= -50
                    && aabox.x.end <= 51
                    && aabox.y.start >= -50
                    && aabox.y.end <= 51
                    && aabox.z.start >= -50
                    && aabox.z.end <= 51
            }
        })
        .fold(
            Box::new(Nothing) as Box<dyn CSGObject>,
            |acc, instruction| match instruction {
                Instruction::On(aabox) => acc + Box::new(aabox),
                Instruction::Off(aabox) => acc - Box::new(aabox),
            },
        );
    let volume = result.volume();
    dbg!(volume);

    Ok(())
}

fn parse_input() -> Result<Vec<Instruction>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$"
        )
        .unwrap();
    }

    input_lines()?
        .map(|line| {
            let line = line?;

            let captures = RE.captures(&line).context("Invalid input line")?;

            let on = captures.get(1).unwrap().as_str() == "on";

            let x = (captures.get(2).unwrap(), captures.get(3).unwrap());
            let x: (i32, i32) = (x.0.as_str().parse()?, x.1.as_str().parse()?);
            let x = x.0..x.1 + 1;

            let y = (captures.get(4).unwrap(), captures.get(5).unwrap());
            let y: (i32, i32) = (y.0.as_str().parse()?, y.1.as_str().parse()?);
            let y = y.0..y.1 + 1;

            let z = (captures.get(6).unwrap(), captures.get(7).unwrap());
            let z: (i32, i32) = (z.0.as_str().parse()?, z.1.as_str().parse()?);
            let z = z.0..z.1 + 1;

            Ok(if on {
                Instruction::On(AABox { x, y: y, z: z })
            } else {
                Instruction::Off(AABox { x, y: y, z: z })
            })
        })
        .collect()
}
