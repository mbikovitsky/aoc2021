const BINGO_ROWS: usize = 5;
const BINGO_COLS: usize = 5;
const BINGO_CELLS: usize = BINGO_ROWS * BINGO_COLS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    numbers: [u32; BINGO_CELLS],
    marked: [bool; BINGO_CELLS],
}

impl Board {
    pub fn new(numbers: &[u32; BINGO_CELLS]) -> Self {
        Self {
            numbers: *numbers,
            marked: Default::default(),
        }
    }

    pub fn mark_number(&self, number: u32) -> Self {
        let mut new_board = *self;

        for index in 0..new_board.numbers.len() {
            if new_board.numbers[index] == number {
                new_board.marked[index] = true;
            }
        }

        new_board
    }

    pub fn is_marked(&self, row: usize, col: usize) -> bool {
        assert!(row < BINGO_ROWS);
        assert!(col < BINGO_COLS);

        self.marked[row * BINGO_ROWS + col]
    }

    pub fn is_winning(&self) -> bool {
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

    pub fn sum_unmarked(&self) -> u32 {
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
pub struct Game {
    pub numbers: Vec<u32>,
    pub boards: Vec<Board>,
}
