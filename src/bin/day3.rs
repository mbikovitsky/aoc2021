use anyhow::Result;

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let numbers = parse_input()?;

    power_consumption(&mut numbers.clone());

    Ok(())
}

fn parse_input() -> Result<Vec<u16>> {
    input_lines()?
        .map(|maybe_line| {
            let line = maybe_line?;
            Ok(u16::from_str_radix(&line, 2)?)
        })
        .collect()
}

fn count_binary_digits(number: u16) -> u32 {
    u16::BITS - number.leading_zeros()
}

fn power_consumption(numbers: &mut [u16]) {
    // Assume that the leading digit in the original input is not 0
    let digits = count_binary_digits(*numbers.iter().max().unwrap());

    let mut gamma_rate = 0u16;
    for digit in 1..digits + 1 {
        let mask = 1 << (digits - digit);

        let (_, &mut median, _) =
            numbers.select_nth_unstable_by_key((numbers.len() - 1) / 2, |&number| number & mask);

        gamma_rate <<= 1;
        gamma_rate |= ((median & mask) != 0) as u16;
    }
    dbg!(gamma_rate);

    let epsilon_rate = (!gamma_rate) & ((1 << digits) - 1);
    dbg!(epsilon_rate);

    let power_consumption = gamma_rate as u32 * epsilon_rate as u32;
    dbg!(power_consumption);
}
