use std::ops::{SubAssign, Sub, AddAssign, Add};

use num::{CheckedAdd, CheckedMul, CheckedSub, Integer};

use crate::{r#box::Box, swiss_box::SwissBox};

#[derive(Debug, Clone)]
pub struct SwissBoxForest<T: Integer> {
    boxen: Vec<SwissBox<T>>,
}

impl<T: Integer> SwissBoxForest<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T: Integer> Default for SwissBoxForest<T> {
    fn default() -> Self {
        Self {
            boxen: Default::default(),
        }
    }
}

impl<T: Integer + Clone> Sub<Box<T>> for SwissBoxForest<T> {
    type Output = Self;

    fn sub(self, rhs: Box<T>) -> Self::Output {
        self - &rhs
    }
}

impl<'a, T: Integer + Clone> Sub<&'a Box<T>> for SwissBoxForest<T> {
    type Output = Self;

    fn sub(mut self, rhs: &'a Box<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Integer + Clone> SubAssign<Box<T>> for SwissBoxForest<T> {
    fn sub_assign(&mut self, rhs: Box<T>) {
        *self -= &rhs;
    }
}

impl<'a, T: Integer + Clone> SubAssign<&'a Box<T>> for SwissBoxForest<T> {
    fn sub_assign(&mut self, rhs: &'a Box<T>) {
        for existing_box in &mut self.boxen {
            *existing_box -= rhs;
        }
    }
}

impl<T: Integer + Clone> Add<Box<T>> for SwissBoxForest<T> {
    type Output =  Self;

    fn add(mut self, rhs: Box<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Integer + Clone> AddAssign<Box<T>> for SwissBoxForest<T> {
    fn add_assign(&mut self, rhs: Box<T>) {
        *self -= &rhs;
        self.boxen.push(rhs.into());
    }
}

impl<T: Integer + CheckedAdd + CheckedSub + CheckedMul> SwissBoxForest<T> {
    pub fn volume(&self) -> Option<T> {
        let mut volume: T = T::zero();

        for box_volume in self.boxen.iter().map(|swiss_box| swiss_box.volume()) {
            volume = volume.checked_add(&box_volume?)?
        }

        Some(volume)
    }
}
