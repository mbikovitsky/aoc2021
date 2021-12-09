use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use anyhow::{Context, Result};
use itertools::Itertools;

use aoc2021::util::input_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HeightMap {
    data: Vec<u8>,
    width: usize,
}

impl HeightMap {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.data.len() / self.width()
    }

    fn get(&self, pos: &Position) -> u8 {
        assert!(pos.row < self.height());
        assert!(pos.col < self.width());

        self.data[pos.row * self.width() + pos.col]
    }

    fn all_points(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.height()).flat_map(|row| (0..self.width()).map(move |col| Position { row, col }))
    }

    fn neighbours(&self, pos: &Position) -> impl Iterator<Item = Position> {
        let pos = *pos;

        assert!(pos.row < self.height());
        assert!(pos.col < self.width());

        let min_row = pos.row.saturating_sub(1);
        let max_row = (pos.row + 1).min(self.height() - 1);

        let up_down = (min_row..=max_row)
            .filter(move |row| *row != pos.row)
            .map(move |row| Position { row, col: pos.col });

        let min_col = pos.col.saturating_sub(1);
        let max_col = (pos.col + 1).min(self.width() - 1);

        let left_right = (min_col..=max_col)
            .filter(move |col| *col != pos.col)
            .map(move |col| Position { col, row: pos.row });

        up_down.chain(left_right)
    }

    fn gradient_direction(&self, pos: &Position) -> Position {
        let neighbour = self
            .neighbours(pos)
            .min_by_key(|neighbour| self.get(neighbour))
            .unwrap_or(*pos);
        if self.get(&neighbour) <= self.get(pos) {
            neighbour
        } else {
            *pos
        }
    }

    fn low_points(&self) -> impl Iterator<Item = Position> + '_ {
        self.all_points().filter(|point| self.is_low_point(point))
    }

    fn is_low_point(&self, pos: &Position) -> bool {
        assert!(pos.row < self.height());
        assert!(pos.col < self.width());

        self.neighbours(pos)
            .all(|neighbour| self.get(&neighbour) > self.get(pos))
    }

    fn risk_level(&self, pos: &Position) -> u16 {
        assert!(self.is_low_point(pos));

        let height: u16 = self.get(pos).into();
        height + 1
    }

    fn basins(&self) -> HashMap<Position, HashSet<Position>> {
        let mut result = HashMap::new();

        for low_point in self.low_points() {
            result.insert(low_point, HashSet::new());
        }

        let mut cache = HashMap::new();

        for point in self.all_points() {
            if self.get(&point) == 9 {
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

    Ok(HeightMap { data, width })
}

fn parse_line(line: &str) -> impl Iterator<Item = u8> + '_ {
    line.chars().map(|c| {
        c.to_digit(10)
            .with_context(|| format!("{} is not a base 10 digit", c))
            .unwrap() as u8
    })
}
