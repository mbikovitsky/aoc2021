use anyhow::Result;

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let mut numbers = parse_input()?;

    power_consumption(&mut numbers);

    life_support_rating(&mut numbers);

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

fn life_support_rating(numbers: &mut [u16]) {
    // Assume that the leading digit in the original input is not 0
    let digits = count_binary_digits(*numbers.iter().max().unwrap());

    let sort_by_digit = |numbers, digit| sort_by_digit(numbers, digit, digits);

    let (zeroes, ones) = sort_by_digit(numbers, 1);
    let (mut oxygen, mut co2) = if ones.len() >= zeroes.len() {
        (ones, zeroes)
    } else {
        (zeroes, ones)
    };

    for digit in 2..digits + 1 {
        if oxygen.len() > 1 {
            let (zeroes, ones) = sort_by_digit(oxygen, digit);
            if ones.len() >= zeroes.len() {
                oxygen = ones;
            } else {
                oxygen = zeroes;
            }
        }

        if co2.len() > 1 {
            let (zeroes, ones) = sort_by_digit(co2, digit);
            if zeroes.len() <= ones.len() {
                co2 = zeroes;
            } else {
                co2 = ones;
            }
        }
    }

    assert!(oxygen.len() == 1);
    let oxygen_generator_rating = oxygen[0];
    dbg!(oxygen_generator_rating);

    assert!(co2.len() == 1);
    let co2_scrubber_rating = co2[0];
    dbg!(co2_scrubber_rating);

    let life_support_rating = oxygen_generator_rating as u32 * co2_scrubber_rating as u32;
    dbg!(life_support_rating);
}

fn sort_by_digit(numbers: &mut [u16], digit: u32, total_digits: u32) -> (&mut [u16], &mut [u16]) {
    let mask = 1 << (total_digits - digit);

    numbers.sort_unstable_by_key(|&number| number & mask);

    let partition = numbers.partition_point(|&number| (number & mask) == 0);

    numbers.split_at_mut(partition)
}
