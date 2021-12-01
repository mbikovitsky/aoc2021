use anyhow::Result;

const INPUT: &str = include_str!("day1.txt");

fn main() -> Result<()> {
    let depths = parse_input()?;

    let depth_increases = (1..depths.len())
        .filter(|&index| depths[index] > depths[index - 1])
        .count();
    dbg!(depth_increases);

    Ok(())
}

fn parse_input() -> Result<Vec<u32>> {
    let result: Result<Vec<_>> = INPUT.lines().map(|line| Ok(line.parse()?)).collect();
    let result = result?;
    Ok(result)
}
