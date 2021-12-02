use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let depths = parse_input()?;

    let depth_increases = (1..depths.len())
        .filter(|&index| depths[index] > depths[index - 1])
        .count();
    dbg!(depth_increases);

    // A B C D E F G
    // A <-> D
    // B <-> E
    // C <-> F

    let window_sum_increases = (3..depths.len())
        .filter(|&index| depths[index] > depths[index - 3])
        .count();
    dbg!(window_sum_increases);

    Ok(())
}

fn parse_input() -> Result<Vec<u32>> {
    let result: Result<Vec<_>> = INPUT.lines().map(|line| Ok(line.parse()?)).collect();
    let result = result?;
    Ok(result)
}
