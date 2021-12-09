use anyhow::{Context, Result};

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
        assert!(pos.row < self.width());
        assert!(pos.col < self.height());

        self.data[pos.row * self.width() + pos.col]
    }

    fn all_points(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.height()).flat_map(|row| (0..self.width()).map(move |col| Position { row, col }))
    }

    fn neighbours(&self, pos: &Position) -> impl Iterator<Item = Position> {
        let pos = *pos;

        assert!(pos.row < self.width());
        assert!(pos.col < self.height());

        let min_row = pos.row.saturating_sub(1);
        let max_row = (pos.row + 1).min(self.height() - 1);

        let min_col = pos.col.saturating_sub(1);
        let max_col = (pos.col + 1).min(self.width() - 1);

        (min_row..=max_row)
            .flat_map(move |row| (min_col..=max_col).map(move |col| Position { row, col }))
            .filter(move |neighbour| neighbour != &pos)
    }

    fn low_points(&self) -> impl Iterator<Item = Position> + '_ {
        self.all_points().filter(|point| self.is_low_point(point))
    }

    fn is_low_point(&self, pos: &Position) -> bool {
        assert!(pos.row < self.width());
        assert!(pos.col < self.height());

        self.neighbours(pos)
            .all(|neighbour| self.get(&neighbour) > self.get(pos))
    }

    fn risk_level(&self, pos: &Position) -> u16 {
        assert!(self.is_low_point(pos));

        let height: u16 = self.get(pos).into();
        height + 1
    }
}

fn main() -> Result<()> {
    let map = parse_input()?;

    let risk_sum: u32 = map
        .low_points()
        .map(|point| map.risk_level(&point) as u32)
        .sum();
    dbg!(risk_sum);

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
