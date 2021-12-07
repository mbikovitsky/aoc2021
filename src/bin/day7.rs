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

    let optimal_position_demo2 = find_optimal_position2(&demo_positions);
    dbg!(optimal_position_demo2);

    let fuel_demo2 = calculate_total_fuel2(&demo_positions, optimal_position_demo2);
    dbg!(fuel_demo2);

    let optimal_position2 = find_optimal_position2(&mut positions);
    dbg!(optimal_position2);

    let fuel2 = calculate_total_fuel2(&positions, optimal_position2);
    dbg!(fuel2);

    Ok(())
}

fn find_optimal_position(positions: &mut [u32]) -> u32 {
    let (_, &mut median, _) = positions.select_nth_unstable((positions.len() - 1) / 2);
    median
}

fn find_optimal_position2(positions: &[u32]) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .min_by_key(|candidate| calculate_total_fuel2(positions, *candidate))
        .unwrap()
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

fn calculate_total_fuel2(positions: &[u32], target: u32) -> u32 {
    positions
        .iter()
        .map(|&position| {
            let distance = if position >= target {
                position - target
            } else {
                target - position
            };

            let fuel = (0 + distance) * (distance + 1) / 2;

            fuel
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
