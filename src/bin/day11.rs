use std::{collections::HashSet, fmt::Display};

use anyhow::Result;

use aoc2021::{
    matrix::{Matrix, Position},
    util::input_lines,
};

const GRID_ROWS: usize = 10;
const GRID_COLS: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    data: Matrix<u8>,
}

impl Grid {
    fn step(&self) -> (Grid, usize) {
        let mut new_grid = self.clone();

        for point in new_grid.data.all_points() {
            *new_grid.data.get_mut(&point) += 1;
        }

        let mut flashed = HashSet::new();
        let mut done = false;
        while !done {
            done = true;

            for point in new_grid.data.all_points() {
                if *new_grid.data.get(&point) <= 9 {
                    continue;
                }

                if !flashed.insert(point) {
                    continue;
                }

                done = false;

                for neighbour in new_grid.data.neighbours_with_diagonals(&point) {
                    *new_grid.data.get_mut(&neighbour) += 1;
                }
            }
        }

        let total_flashes = flashed.len();

        for point in flashed.into_iter() {
            *new_grid.data.get_mut(&point) = 0;
        }

        (new_grid, total_flashes)
    }

    fn sync_point(&self) -> usize {
        let mut current = self.clone();
        for step in 1usize.. {
            let (next, flashes) = current.step();
            if flashes == GRID_ROWS * GRID_COLS {
                return step;
            }
            current = next;
        }
        panic!("Solution not found")
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS {
                write!(f, "{}", self.data.get(&Position { row, col }))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let grid = parse_input()?;

    let (_, total_flashes) = (0..100).fold((grid.clone(), 0), |acc, _| {
        let (next, flashes) = acc.0.step();
        (next, acc.1 + flashes)
    });
    dbg!(total_flashes);

    let sync_point = grid.sync_point();
    dbg!(sync_point);

    Ok(())
}

fn parse_input() -> Result<Grid> {
    let mut octopii = vec![];

    for line in input_lines()? {
        let line = line?;

        octopii.extend(line.chars().map(|c| c.to_digit(10).unwrap() as u8));
        assert!(octopii.len() % GRID_COLS == 0);
    }
    assert_eq!(octopii.len(), GRID_ROWS * GRID_COLS);

    Ok(Grid {
        data: Matrix::new(octopii, GRID_COLS),
    })
}
