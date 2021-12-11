use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub struct Matrix<T> {
    data: Vec<T>,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<T>, cols: usize) -> Self {
        assert_eq!(data.len() % cols, 0);
        Self { data, cols }
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.data.len() / self.cols()
    }

    pub fn get(&self, pos: &Position) -> &T {
        assert!(pos.row < self.rows());
        assert!(pos.col < self.cols());

        &self.data[pos.row * self.cols() + pos.col]
    }

    pub fn get_mut(&mut self, pos: &Position) -> &mut T {
        assert!(pos.row < self.rows());
        assert!(pos.col < self.cols());

        let cols = self.cols();

        &mut self.data[pos.row * cols + pos.col]
    }

    pub fn all_points(&self) -> impl Iterator<Item = Position> {
        let rows = self.rows();
        let cols = self.cols();
        (0..rows).flat_map(move |row| (0..cols).map(move |col| Position { row, col }))
    }

    pub fn neighbours(&self, pos: &Position) -> impl Iterator<Item = Position> {
        let pos = *pos;

        assert!(pos.row < self.rows());
        assert!(pos.col < self.cols());

        let min_row = pos.row.saturating_sub(1);
        let max_row = (pos.row + 1).min(self.rows() - 1);

        let up_down = (min_row..=max_row)
            .filter(move |row| *row != pos.row)
            .map(move |row| Position { row, col: pos.col });

        let min_col = pos.col.saturating_sub(1);
        let max_col = (pos.col + 1).min(self.cols() - 1);

        let left_right = (min_col..=max_col)
            .filter(move |col| *col != pos.col)
            .map(move |col| Position { col, row: pos.row });

        up_down.chain(left_right)
    }

    pub fn neighbours_with_diagonals(&self, pos: &Position) -> impl Iterator<Item = Position> {
        let pos = *pos;

        assert!(pos.row < self.rows());
        assert!(pos.col < self.cols());

        let min_row = pos.row.saturating_sub(1);
        let max_row = (pos.row + 1).min(self.rows() - 1);

        let min_col = pos.col.saturating_sub(1);
        let max_col = (pos.col + 1).min(self.cols() - 1);

        (min_row..=max_row)
            .flat_map(move |row| (min_col..=max_col).map(move |col| Position { row, col }))
            .filter(move |neighbour| neighbour != &pos)
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix")
            .field("data", &self.data)
            .field("cols", &self.cols)
            .finish()
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            cols: self.cols.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.cols == other.cols
    }
}

impl<T: Eq> Eq for Matrix<T> {}

impl<T: Hash> Hash for Matrix<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.cols.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
