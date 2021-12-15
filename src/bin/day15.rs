use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use aoc2021::{
    matrix::{Matrix, Position},
    util::input_lines,
};

fn main() -> Result<()> {
    let grid = parse_input()?;

    let lowest_risk = lowest_risk_astar(&grid, 1);
    dbg!(lowest_risk);

    let lowest_risk2 = lowest_risk_astar(&grid, 5);
    dbg!(lowest_risk2);

    Ok(())
}

fn lowest_risk_astar(grid: &Matrix<u8>, factor: usize) -> u64 {
    // https://en.wikipedia.org/w/index.php?title=A*_search_algorithm&oldid=1060160033#Pseudocode

    let source = Position { row: 0, col: 0 };
    let target = Position {
        row: grid.rows() * factor - 1,
        col: grid.cols() * factor - 1,
    };

    let risk = |position: &Position| {
        let original = Position {
            row: position.row % grid.rows(),
            col: position.col % grid.cols(),
        };
        let original_risk = *grid.get(&original);

        let distance = (position.row - original.row)
            .checked_add(position.col - original.col)
            .unwrap();

        let risk = ((original_risk as usize + distance) % 9) as u8;
        let risk = if risk == 0 { 9 } else { risk };

        risk
    };

    let heuristic = |position: &Position| -> u64 {
        ((target.row - position.row) + (target.col - position.col))
            .try_into()
            .unwrap()
    };

    let neighbours = |pos: Position| {
        let min_row = pos.row.saturating_sub(1);
        let max_row = (pos.row + 1).min(target.row);

        let up_down = (min_row..=max_row)
            .filter(move |row| *row != pos.row)
            .map(move |row| Position { row, col: pos.col });

        let min_col = pos.col.saturating_sub(1);
        let max_col = (pos.col + 1).min(target.col);

        let left_right = (min_col..=max_col)
            .filter(move |col| *col != pos.col)
            .map(move |col| Position { col, row: pos.row });

        up_down.chain(left_right)
    };

    let mut open_set = HashSet::from([source]);
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::from([(source, 0u64)]);
    let mut f_score = HashMap::from([(source, heuristic(&source))]);

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by_key(|node| f_score.get(node).unwrap_or(&u64::MAX))
            .unwrap();

        if current == target {
            let mut current = current;

            let mut total_cost: u64 = risk(&current).into();
            while let Some(prev) = came_from.get(&current) {
                current = *prev;
                total_cost += risk(&current) as u64;
            }
            return total_cost - risk(&source) as u64;
        }

        open_set.remove(&current);

        for neighbour in neighbours(current) {
            let tentative_gscore = *g_score.get(&current).unwrap() + risk(&neighbour) as u64;
            if tentative_gscore < *g_score.get(&neighbour).unwrap_or(&u64::MAX) {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_gscore);
                f_score.insert(neighbour, tentative_gscore + heuristic(&neighbour));
                if !open_set.contains(&neighbour) {
                    open_set.insert(neighbour);
                }
            }
        }
    }

    u64::MAX
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
