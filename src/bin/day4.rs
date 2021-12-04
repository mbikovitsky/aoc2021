use anyhow::{Context, Result};

use aoc2021::util::input_lines;

const BINGO_ROWS: usize = 5;
const BINGO_COLS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Board {
    numbers: [u32; BINGO_ROWS * BINGO_COLS],
    marked: [bool; BINGO_ROWS * BINGO_COLS],
}

impl Board {
    fn mark_number(&self, number: u32) -> Board {
        let mut new_board = *self;

        for index in 0..new_board.numbers.len() {
            if new_board.numbers[index] == number {
                new_board.marked[index] = true;
            }
        }

        new_board
    }

    fn is_marked(&self, row: usize, col: usize) -> bool {
        assert!(row < BINGO_ROWS);
        assert!(col < BINGO_COLS);

        self.marked[row * BINGO_ROWS + col]
    }

    fn is_winning(&self) -> bool {
        // Check rows
        for row in 0..BINGO_ROWS {
            if self.marked[row * BINGO_ROWS..row * BINGO_ROWS + BINGO_COLS] == [true; BINGO_COLS] {
                return true;
            }
        }

        // Check columns
        for col in 0..BINGO_COLS {
            if (0..BINGO_ROWS).all(|row| self.is_marked(row, col)) {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.marked
            .iter()
            .enumerate()
            .filter_map(|(index, &marked)| {
                if marked {
                    None
                } else {
                    Some(self.numbers[index])
                }
            })
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

fn main() -> Result<()> {
    let game = parse_input()?;

    let score = find_first_winning_board_score(game).unwrap();
    dbg!(score);

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

        let board = Board {
            numbers: numbers.try_into().unwrap(),
            marked: Default::default(),
        };

        boards.push(board);
    }

    Ok(Game { numbers, boards })
}
