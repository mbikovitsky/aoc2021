use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("day1.txt");

fn main() -> Result<()> {
    let depths = parse_input()?;

    let depth_increases = (1..depths.len())
        .filter(|&index| depths[index] > depths[index - 1])
        .count();
    dbg!(depth_increases);

    let window_sum_increases = depths
        .iter()
        .tuple_windows()
        .map(|window: (_, _, _)| window.0 + window.1 + window.2)
        .tuple_windows()
        .filter(|sums: &(_, _)| sums.1 > sums.0)
        .count();
    dbg!(window_sum_increases);

    Ok(())
}

fn parse_input() -> Result<Vec<u32>> {
    let result: Result<Vec<_>> = INPUT.lines().map(|line| Ok(line.parse()?)).collect();
    let result = result?;
    Ok(result)
}
