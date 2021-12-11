use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use anyhow::{Context, Result};
use itertools::Itertools;

use aoc2021::{
    matrix::{Matrix, Position},
    util::input_lines,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HeightMap {
    data: Matrix<u8>,
}

impl HeightMap {
    fn gradient_direction(&self, pos: &Position) -> Position {
        let neighbour = self
            .data
            .neighbours(pos)
            .min_by_key(|neighbour| self.data.get(neighbour))
            .unwrap_or(*pos);
        if self.data.get(&neighbour) <= self.data.get(pos) {
            neighbour
        } else {
            *pos
        }
    }

    fn low_points(&self) -> impl Iterator<Item = Position> + '_ {
        self.data
            .all_points()
            .filter(|point| self.is_low_point(point))
    }

    fn is_low_point(&self, pos: &Position) -> bool {
        self.data
            .neighbours(pos)
            .all(|neighbour| self.data.get(&neighbour) > self.data.get(pos))
    }

    fn risk_level(&self, pos: &Position) -> u16 {
        assert!(self.is_low_point(pos));

        let height: u16 = (*self.data.get(pos)).into();
        height + 1
    }

    fn basins(&self) -> HashMap<Position, HashSet<Position>> {
        let mut result = HashMap::new();

        for low_point in self.low_points() {
            result.insert(low_point, HashSet::new());
        }

        let mut cache = HashMap::new();

        for point in self.data.all_points() {
            if *self.data.get(&point) == 9 {
                continue;
            }

            let mut path = vec![point];
            let mut current = point;
            while !result.contains_key(&current) {
                if let Some(low_point) = cache.get(&current) {
                    current = *low_point;
                } else {
                    current = self.gradient_direction(&current);
                }
                path.push(current);
            }

            result.get_mut(&current).unwrap().extend(&path);

            for step in path {
                cache.insert(step, current);
            }
        }

        result
    }
}

fn main() -> Result<()> {
    let map = parse_input()?;

    let risk_sum: u32 = map
        .low_points()
        .map(|point| map.risk_level(&point) as u32)
        .sum();
    dbg!(risk_sum);

    let basins = map.basins();
    let basin_sizes: Vec<_> = basins
        .into_values()
        .map(|basin| basin.len())
        .sorted_unstable_by_key(|len| Reverse(*len))
        .collect();
    let size_product: usize = basin_sizes[0..3].iter().product();
    dbg!(size_product);

    Ok(())
}

fn parse_input() -> Result<HeightMap> {
    let mut data = vec![];

    let mut lines = input_lines()?;

    let first_line = lines.next().context("Input file is empty")??;
    data.extend(parse_line(&first_line));
    let width = data.len();

    for line in lines {
        let line = line?;
        data.extend(parse_line(&line));
    }

    Ok(HeightMap {
        data: Matrix::new(data, width),
    })
}

fn parse_line(line: &str) -> impl Iterator<Item = u8> + '_ {
    line.chars().map(|c| {
        c.to_digit(10)
            .with_context(|| format!("{} is not a base 10 digit", c))
            .unwrap() as u8
    })
}
