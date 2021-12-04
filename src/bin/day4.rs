use anyhow::{Context, Result};

use aoc2021::{
    bingo::{Board, Game},
    util::input_lines,
};

fn main() -> Result<()> {
    let game = parse_input()?;

    let first_score = find_first_winning_board_score(game.clone()).unwrap();
    dbg!(first_score);

    let last_score = find_last_winning_board_score(game).unwrap();
    dbg!(last_score);

    Ok(())
}

fn find_first_winning_board_score(mut game: Game) -> Option<u32> {
    for &number in &game.numbers {
        for board in &mut game.boards {
            *board = board.mark_number(number);
            if board.is_winning() {
                return Some(board.sum_unmarked() * number);
            }
        }
    }

    None
}

fn find_last_winning_board_score(mut game: Game) -> Option<u32> {
    let mut last_winning_board_score = None;

    for &number in &game.numbers {
        for board in &mut game.boards {
            if board.is_winning() {
                continue;
            }

            *board = board.mark_number(number);
            if board.is_winning() {
                last_winning_board_score = Some(board.sum_unmarked() * number);
            }
        }
    }

    last_winning_board_score
}

fn parse_input() -> Result<Game> {
    let mut lines = input_lines()?;

    let numbers_line = lines
        .next()
        .context("Empty input file")?
        .context("Error reading numbers order line")?;
    let numbers: Result<Vec<_>> = numbers_line
        .split(',')
        .map(|number| Ok(number.parse()?))
        .collect();
    let numbers = numbers?;

    let mut boards = Vec::new();
    while let Some(separator) = lines.next() {
        assert!(separator
            .context("Error reading separator line")?
            .is_empty());

        let board_lines: Result<Vec<String>> = lines.by_ref().take(5).collect();
        let board_lines = board_lines?;
        assert_eq!(board_lines.len(), 5);

        let numbers: Result<Vec<u32>> = board_lines
            .into_iter()
            .flat_map(|board_line| {
                board_line
                    .split_whitespace()
                    .map(|number| Ok(number.parse()?))
                    .collect::<Vec<Result<u32>>>()
            })
            .collect();
        let numbers = numbers?;
        assert_eq!(numbers.len(), 25);

        let board = Board::new(&numbers.try_into().unwrap());

        boards.push(board);
    }

    Ok(Game { numbers, boards })
}
