use anyhow::Result;

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let mut positions = parse_input()?;

    let mut demo_positions = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    let optimal_position_demo = find_optimal_position(&mut demo_positions);
    dbg!(optimal_position_demo);

    let fuel_demo = calculate_total_distance(&demo_positions, optimal_position_demo);
    dbg!(fuel_demo);

    let optimal_position = find_optimal_position(&mut positions);
    dbg!(optimal_position);

    let fuel = calculate_total_distance(&positions, optimal_position);
    dbg!(fuel);

    Ok(())
}

fn find_optimal_position(positions: &mut [u32]) -> u32 {
    let (_, &mut median, _) = positions.select_nth_unstable((positions.len() - 1) / 2);
    median
}

fn calculate_total_distance(positions: &[u32], target: u32) -> u32 {
    positions
        .iter()
        .map(|&position| {
            if position >= target {
                position - target
            } else {
                target - position
            }
        })
        .sum()
}

fn parse_input() -> Result<Vec<u32>> {
    let lines: Result<Vec<_>> = input_lines()?.collect();
    let lines = lines?;
    assert_eq!(lines.len(), 1);

    let positions: Result<Vec<_>> = lines[0]
        .split(',')
        .map(|counter| Ok(counter.parse()?))
        .collect();
    let counters = positions?;

    Ok(counters)
}
