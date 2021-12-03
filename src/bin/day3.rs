use anyhow::Result;

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let numbers = parse_input()?;

    // Assume that the leading digit is not 0
    let digits = count_binary_digits(*numbers.iter().max().unwrap());

    let ones = count_ones(&numbers);
    let ones = &ones[0..digits as usize];

    let mut gamma_rate = 0u16;
    for count in ones.iter().rev().copied() {
        gamma_rate <<= 1;
        gamma_rate |= (count > numbers.len() / 2) as u16;
    }
    dbg!(gamma_rate);

    let epsilon_rate = (!gamma_rate) & ((1 << digits) - 1);
    dbg!(epsilon_rate);

    let power_consumption = gamma_rate as u32 * epsilon_rate as u32;
    dbg!(power_consumption);

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

fn count_ones(numbers: &[u16]) -> [usize; u16::BITS as usize] {
    let mut counters = [0usize; u16::BITS as usize];

    for number in numbers {
        let mut temp = *number;
        let mut digit = 0;
        while temp > 0 {
            counters[digit] += (temp & 1) as usize;
            temp >>= 1;
            digit += 1;
        }
    }

    counters
}

fn count_binary_digits(number: u16) -> u32 {
    u16::BITS - number.leading_zeros()
}
