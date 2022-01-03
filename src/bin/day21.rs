use anyhow::{Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use aoc2021::util::input_lines;

fn main() -> Result<()> {
    let positions = parse_input()?;

    let (player_1_wins, winning_turns) = find_winner(positions.0, positions.1);

    let loser_score = if player_1_wins {
        player_score(winning_turns - 1, player_2_position(positions.1))
    } else {
        player_score(winning_turns, player_1_position(positions.0))
    };

    let dice_rolls = if player_1_wins {
        (2 * winning_turns - 1) * 3
    } else {
        2 * winning_turns * 3
    };

    dbg!(loser_score * dice_rolls);

    Ok(())
}

fn find_winner(player_1_initial: u8, player_2_initial: u8) -> (bool, u32) {
    let player_1_turns = turns_to_win(player_1_position(player_1_initial));
    let player_2_turns = turns_to_win(player_2_position(player_2_initial));

    if player_1_turns <= player_2_turns {
        (true, player_1_turns)
    } else {
        (false, player_2_turns)
    }
}

fn player_1_position(initial_position: u8) -> impl Copy + Fn(u32) -> u32 {
    move |turns| ((initial_position as u32 + 9 * turns * turns - 3 * turns) % 10)
}

fn player_2_position(initial_position: u8) -> impl Copy + Fn(u32) -> u32 {
    move |turns| ((initial_position as u32 + 9 * turns * turns + 6 * turns) % 10)
}

fn player_score(turns: u32, mut position: impl FnMut(u32) -> u32) -> u32 {
    let scores = {
        let mut positions = [0u32; 10];
        for n in 1..=10u32 {
            positions[n as usize - 1] = position(n) + 1;
        }
        positions
    };

    (turns / 10) * scores.iter().sum::<u32>()
        + scores.iter().take((turns % 10) as usize).sum::<u32>()
}

fn turns_to_win(position: impl Copy + FnMut(u32) -> u32) -> u32 {
    // https://en.wikipedia.org/w/index.php?title=Binary_search_algorithm&oldid=1062988272#Procedure_for_finding_the_leftmost_element
    let mut l = 0u32;
    let mut r = 1000u32;
    while l < r {
        let m = (l + r) / 2;
        if player_score(m, position) < 1000 {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn parse_input() -> Result<(u8, u8)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Player \d+ starting position: (\d+)$").unwrap();
    }

    let positions: (_, _) = input_lines()?
        .map(|line| -> Result<u8> {
            let line = line?;

            let captures = RE.captures(&line).context("Invalid input line")?;

            let position = captures.get(1).unwrap();
            let position: u8 = position.as_str().parse()?;

            Ok(position - 1)
        })
        .collect_tuple()
        .context("Invalid number of lines")?;

    Ok((positions.0?, positions.1?))
}
