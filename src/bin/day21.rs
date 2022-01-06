use std::cmp::max;

use anyhow::{Context, Result};
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use ndarray::{Array5, ArrayView5};
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

    let lookup = generate_quantum_score_lookup_table();
    let wins_1 = winning_options_player_1(positions.0, positions.1, lookup.view());
    let wins_2 = winning_options_player_2(positions.0, positions.1, lookup.view());

    dbg!(wins_1);
    dbg!(wins_2);
    dbg!(max(wins_1, wins_2));

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

const WINNING_SCORE: usize = 21;
const POSITIONS: usize = 10;
const MAX_TURNS_TO_WINNING_SCORE: usize = 14;

fn generate_quantum_score_lookup_table() -> Array5<u64> {
    let mut table = Array5::zeros((
        WINNING_SCORE,
        WINNING_SCORE,
        POSITIONS,
        POSITIONS,
        MAX_TURNS_TO_WINNING_SCORE,
    ));

    for m in 1..=WINNING_SCORE {
        for M in m + 1..=WINNING_SCORE {
            for s in 0..POSITIONS {
                for f in 0..POSITIONS {
                    if f + 1 < m || f + 1 >= M {
                        continue;
                    }

                    table[[m - 1, M - 1, s, f, 0]] += throw_combos(
                        (f + (-(s as isize)).rem_euclid(POSITIONS as isize) as usize) % POSITIONS,
                    );
                }
            }
        }
    }

    for m in 1..=21 {
        for M in m + 1..=21 {
            for s in 0..POSITIONS {
                for f in 0..POSITIONS {
                    for t in 2..=MAX_TURNS_TO_WINNING_SCORE {
                        for d in 3..=9 {
                            let p = (s + d) % POSITIONS;

                            let lookup_m = if m > p + 1 { m - (p + 1) } else { 1 };

                            let lookup_M = if M > p + 1 {
                                M - (p + 1)
                            } else {
                                continue;
                            };

                            table[[m - 1, M - 1, s, f, t - 1]] +=
                                throw_combos(d) * table[[lookup_m - 1, lookup_M - 1, p, f, t - 2]];
                        }
                    }
                }
            }
        }
    }

    table
}

fn winning_options_player_1(initial_1: u8, initial_2: u8, lookup: ArrayView5<u64>) -> u64 {
    (3u8..=MAX_TURNS_TO_WINNING_SCORE.try_into().unwrap())
        .map(|t| winning_options_player_1_turns(initial_1, initial_2, t, lookup))
        .sum()
}

fn winning_options_player_2(initial_1: u8, initial_2: u8, lookup: ArrayView5<u64>) -> u64 {
    (3u8..=MAX_TURNS_TO_WINNING_SCORE.try_into().unwrap())
        .map(|t| winning_options_player_2_turns(initial_1, initial_2, t, lookup))
        .sum()
}

fn winning_options_player_1_turns(
    initial_1: u8,
    initial_2: u8,
    turns: u8,
    lookup: ArrayView5<u64>,
) -> u64 {
    let ways_to_win: u64 = iproduct!(0..POSITIONS, 3..=9usize)
        .map(|(f, d)| {
            throw_combos(d)
                * lookup[[
                    WINNING_SCORE - f - 2,
                    WINNING_SCORE - 1,
                    initial_1.into(),
                    (f + (-(d as isize)).rem_euclid(POSITIONS as isize) as usize) % POSITIONS,
                    (turns - 2).into(),
                ]]
        })
        .sum();

    let ways_to_lose: u64 = (0..POSITIONS)
        .map(|f| lookup[[1 - 1, 21 - 1, initial_2 as usize, f, (turns - 2).into()]])
        .sum();

    ways_to_win * ways_to_lose
}

fn winning_options_player_2_turns(
    initial_1: u8,
    initial_2: u8,
    turns: u8,
    lookup: ArrayView5<u64>,
) -> u64 {
    let ways_to_win: u64 = iproduct!(0..POSITIONS, 3..=9usize)
        .map(|(f, d)| {
            throw_combos(d)
                * lookup[[
                    WINNING_SCORE - f - 2,
                    WINNING_SCORE - 1,
                    initial_2.into(),
                    (f + (-(d as isize)).rem_euclid(POSITIONS as isize) as usize) % POSITIONS,
                    (turns - 2).into(),
                ]]
        })
        .sum();

    let ways_to_lose: u64 = (0..POSITIONS)
        .map(|f| lookup[[1 - 1, 21 - 1, initial_1 as usize, f, (turns - 1).into()]])
        .sum();

    ways_to_win * ways_to_lose
}

/// How many ways are there to obtain a given `distance` using three throws of d3.
fn throw_combos(distance: usize) -> u64 {
    static COMBOS: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];

    if distance < 3 || distance > 9 {
        0
    } else {
        COMBOS[distance - 3]
    }
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
