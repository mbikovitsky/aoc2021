use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use aoc2021::{
    matrix::{Matrix, Position},
    util::input_lines,
};

fn main() -> Result<()> {
    let grid = parse_input()?;

    let lowest_risk = lowest_risk_path_dijkstra(&grid);
    dbg!(lowest_risk);

    Ok(())
}

fn lowest_risk_path_dijkstra(grid: &Matrix<u8>) -> u64 {
    let source = Position { row: 0, col: 0 };
    let target = Position {
        row: grid.rows() - 1,
        col: grid.cols() - 1,
    };

    let mut vertices: HashSet<Position> = grid.all_points().collect();
    let mut distances = HashMap::from([(source, 0u64)]);
    let mut previous = HashMap::new();

    while !vertices.is_empty() {
        let current = *vertices
            .iter()
            .min_by_key(|vertex| *distances.get(vertex).unwrap_or(&u64::MAX))
            .unwrap();
        vertices.remove(&current);

        if current == target {
            break;
        }

        for neighbour in grid.neighbours(&current) {
            let neighbour_cost: u64 = (*grid.get(&neighbour)).into();
            let alt = *distances.get(&current).unwrap() + neighbour_cost;
            if alt < *distances.get(&neighbour).unwrap_or(&u64::MAX) {
                distances.insert(neighbour, alt);
                previous.insert(neighbour, current);
            }
        }
    }

    let mut total_cost = 0u64;
    let mut current = target;
    while let Some(prev) = previous.get(&current) {
        let cost: u64 = (*grid.get(&current)).into();
        total_cost += cost;
        current = *prev;
    }

    total_cost
}

fn parse_input() -> Result<Matrix<u8>> {
    let mut data = vec![];

    fn add_line(line: &str, data: &mut Vec<u8>) {
        data.extend(line.chars().map(|c| c.to_digit(10).unwrap() as u8));
    }

    let mut lines = input_lines()?;

    let first_line = lines.next().context("Input is empty")??;
    add_line(&first_line, &mut data);
    let cols = data.len();

    for line in lines {
        let line = line?;
        add_line(&line, &mut data);
        assert_eq!(data.len() % cols, 0);
    }
    assert_eq!(data.len() / cols, cols);

    Ok(Matrix::new(data, cols))
}
