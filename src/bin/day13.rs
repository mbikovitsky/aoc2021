use std::collections::HashSet;

use anyhow::{bail, Context, Result};

use aoc2021::util::input_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instructions {
    paper: HashSet<(u32, u32)>,
    folds: Vec<Fold>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    Up(u32),
    Left(u32),
}

fn main() -> Result<()> {
    let instructions = parse_input()?;

    let after_one_fold = fold(&instructions.paper, instructions.folds[0]);
    let dots_after_one_fold = after_one_fold.len();
    dbg!(dots_after_one_fold);

    let mut paper = instructions.paper;
    for instruction in instructions.folds {
        paper = fold(&paper, instruction);
    }
    print_paper(&paper);

    Ok(())
}

fn fold(paper: &HashSet<(u32, u32)>, instruction: Fold) -> HashSet<(u32, u32)> {
    let mut result = HashSet::new();

    for dot in paper.iter() {
        match instruction {
            Fold::Up(row) => {
                assert_ne!(dot.1, row);

                if dot.1 < row {
                    result.insert(*dot);
                } else {
                    result.insert((dot.0, row.checked_sub(dot.1 - row).unwrap()));
                }
            }
            Fold::Left(column) => {
                assert_ne!(dot.0, column);

                if dot.0 < column {
                    result.insert(*dot);
                } else {
                    result.insert((column.checked_sub(dot.0 - column).unwrap(), dot.1));
                }
            }
        }
    }

    result
}

fn print_paper(paper: &HashSet<(u32, u32)>) {
    let max_row = paper.iter().map(|dot| dot.1).max().unwrap_or(0);
    let max_col = paper.iter().map(|dot| dot.0).max().unwrap_or(0);

    for row in 0..=max_row {
        let mut string = String::with_capacity(max_col.checked_add(1).unwrap().try_into().unwrap());
        for col in 0..=max_col {
            if paper.contains(&(col, row)) {
                string.push('#');
            } else {
                string.push('.');
            }
        }
        println!("{}", string);
    }
}

fn parse_input() -> Result<Instructions> {
    let mut lines = input_lines()?;

    let mut paper = HashSet::new();
    let mut folds = vec![];

    loop {
        match lines.next() {
            Some(coordinate_line) => {
                let coodinate_line = coordinate_line?;
                if coodinate_line.is_empty() {
                    break;
                }

                let coordinates = coodinate_line
                    .split_once(',')
                    .context("Invalid coordinates")?;
                let coordinates = (coordinates.0.parse()?, coordinates.1.parse()?);
                paper.insert(coordinates);
            }
            None => {
                return Ok(Instructions {
                    paper,
                    folds: vec![],
                });
            }
        }
    }

    for line in lines {
        let line = line?;

        let instruction_parts = line.split_once('=').context("Invalid fold instruction")?;

        let coordinate: u32 = instruction_parts.1.parse()?;

        let axis = instruction_parts
            .0
            .chars()
            .last()
            .context("Invalid fold instruction")?;

        let fold = match axis {
            'y' => Fold::Up(coordinate),
            'x' => Fold::Left(coordinate),
            _ => bail!("Invalid fold axis '{}'", axis),
        };

        folds.push(fold);
    }

    Ok(Instructions { paper, folds })
}
