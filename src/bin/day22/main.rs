mod r#box;
mod int_interval;
mod swiss_box;
mod swiss_box_forest;

use std::borrow::Borrow;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::util::input_lines;

use crate::{r#box::Box, swiss_box_forest::SwissBoxForest};

#[derive(Debug, Clone)]
enum Instruction {
    On(Box<i64>),
    Off(Box<i64>),
}

fn main() -> Result<()> {
    let instructions = parse_input()?;

    let in_initialization_area = instructions.iter().filter(|instruction| match instruction {
        Instruction::On(r#box) | Instruction::Off(r#box) => {
            r#box.x.start >= -50
                && r#box.x.end <= 51
                && r#box.y.start >= -50
                && r#box.y.end <= 51
                && r#box.z.start >= -50
                && r#box.z.end <= 51
        }
    });
    let init_area_volume = execute_instructions(in_initialization_area);
    dbg!(init_area_volume);

    let full_volume = execute_instructions(instructions);
    dbg!(full_volume);

    Ok(())
}

fn execute_instructions<E: Borrow<Instruction>>(instructions: impl IntoIterator<Item = E>) -> i64 {
    let result =
        instructions
            .into_iter()
            .fold(
                SwissBoxForest::<i64>::new(),
                |forest, instruction| match instruction.borrow() {
                    Instruction::On(r#box) => forest + *r#box,
                    Instruction::Off(r#box) => forest - *r#box,
                },
            );

    result.volume().unwrap()
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
            let x: (i64, i64) = (x.0.as_str().parse()?, x.1.as_str().parse()?);
            let x = (x.0..x.1 + 1).into();

            let y = (captures.get(4).unwrap(), captures.get(5).unwrap());
            let y: (i64, i64) = (y.0.as_str().parse()?, y.1.as_str().parse()?);
            let y = (y.0..y.1 + 1).into();

            let z = (captures.get(6).unwrap(), captures.get(7).unwrap());
            let z: (i64, i64) = (z.0.as_str().parse()?, z.1.as_str().parse()?);
            let z = (z.0..z.1 + 1).into();

            Ok(if on {
                Instruction::On(Box { x, y, z })
            } else {
                Instruction::Off(Box { x, y, z })
            })
        })
        .collect()
}
