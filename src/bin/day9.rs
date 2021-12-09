use anyhow::{Context, Result};

use aoc2021::util::input_lines;

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

    fn get(&self, row: usize, col: usize) -> u8 {
        assert!(row < self.width());
        assert!(col < self.height());

        self.data[row * self.width() + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut u8 {
        assert!(row < self.width());
        assert!(col < self.height());

        let index = row * self.width() + col;

        &mut self.data[index]
    }

    fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        assert!(row < self.width());
        assert!(col < self.height());

        let min_row = row.saturating_sub(1);
        let max_row = (row + 1).min(self.height() - 1);

        let min_col = col.saturating_sub(1);
        let max_col = (col + 1).min(self.width() - 1);

        (min_row..=max_row)
            .flat_map(move |row| (min_col..=max_col).map(move |col| (row, col)))
            .filter(move |(y, x)| !(*y == row && *x == col))
    }

    fn low_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width()).flat_map(move |row| {
            (0..self.height()).filter_map(move |col| {
                if self.is_low_point(row, col) {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
    }

    fn is_low_point(&self, row: usize, col: usize) -> bool {
        assert!(row < self.width());
        assert!(col < self.height());

        self.neighbours(row, col)
            .all(|neighbour| self.get(neighbour.0, neighbour.1) > self.get(row, col))
    }

    fn risk_level(&self, row: usize, col: usize) -> u16 {
        assert!(self.is_low_point(row, col));

        let height: u16 = self.get(row, col).into();
        height + 1
    }
}

fn main() -> Result<()> {
    let map = parse_input()?;

    let risk_sum: u32 = map
        .low_points()
        .map(|point| map.risk_level(point.0, point.1) as u32)
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
