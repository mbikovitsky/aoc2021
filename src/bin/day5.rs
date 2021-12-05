use std::collections::HashMap;

use anyhow::Result;

use aoc2021::{geometry::Line, util::input_lines};

fn main() -> Result<()> {
    let lines = parse_input()?;

    let overlapping_points_horizontal_vertical = count_overlapping_points(
        lines
            .iter()
            .filter(|line| line.horizontal() || line.vertical()),
    );
    dbg!(overlapping_points_horizontal_vertical);

    Ok(())
}

fn count_overlapping_points<I>(lines: I) -> usize
where
    I: IntoIterator,
    I::Item: AsRef<Line>,
{
    let mut counter = HashMap::new();

    for line in lines.into_iter() {
        let line = line.as_ref();

        for point in line.points() {
            if let Some(count) = counter.get_mut(&point) {
                *count += 1usize;
            } else {
                counter.insert(point, 1usize);
            }
        }
    }

    counter.into_values().filter(|count| *count > 1).count()
}

fn parse_input() -> Result<Vec<Line>> {
    input_lines()?.map(|line| Ok(line?.parse()?)).collect()
}
