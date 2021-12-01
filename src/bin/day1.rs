use anyhow::Result;

const INPUT: &str = include_str!("day1.txt");

fn main() -> Result<()> {
    let depths = parse_input()?;

    let increases: u64 = (1..depths.len())
        .map(|index| {
            if depths[index] > depths[index - 1] {
                1
            } else {
                0
            }
        })
        .sum();
    dbg!(increases);

    Ok(())
}

fn parse_input() -> Result<Vec<u32>> {
    let result: Result<Vec<_>> = INPUT.lines().map(|line| Ok(line.parse()?)).collect();
    let result = result?;
    Ok(result)
}
