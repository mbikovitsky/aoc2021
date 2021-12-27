mod ast;
lalrpop_mod!(snailfish, "/bin/day18/snailfish.rs");

use anyhow::Result;
use itertools::Itertools;
use lalrpop_util::lalrpop_mod;

use aoc2021::util::input_lines;

use ast::SnailfishNum;
use snailfish::SnailfishNumParser;

fn main() -> Result<()> {
    let numbers = parse_input();

    let sum = numbers.iter().cloned().reduce(|a, b| a + b).unwrap();
    let magnitude = sum.magnitude();
    dbg!(magnitude);

    let max_magnitude = numbers
        .iter()
        .permutations(2)
        .map(|pair| {
            pair.into_iter()
                .cloned()
                .reduce(|a, b| a + b)
                .unwrap()
                .magnitude()
        })
        .max()
        .unwrap();
    dbg!(max_magnitude);

    Ok(())
}

fn parse_input() -> Vec<SnailfishNum> {
    input_lines()
        .unwrap()
        .map(|line| SnailfishNumParser::new().parse(&line.unwrap()).unwrap())
        .collect()
}
