use std::ops::Range;

use num::{Integer, CheckedSub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IntInterval<T: Integer> {
    pub start: T,
    pub end: T,
}

impl<T: Integer> IntInterval<T> {
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

impl<T: Integer + CheckedSub> IntInterval<T> {
    pub fn len(&self) -> Option<T> {
        if self.is_empty() {
            Some(T::zero())
        } else {
            self.end.checked_sub(&self.start)
        }
    }
}

impl<T: Integer + Clone> IntInterval<T> {
    pub fn intersect(&self, rhs: &Self) -> Self {
        let start = std::cmp::max(&self.start, &rhs.start);
        let end = std::cmp::min(&self.end, &rhs.end);
        Self {
            start: start.clone(),
            end: end.clone(),
        }
    }
}

impl<T: Integer> From<Range<T>> for IntInterval<T> {
    fn from(range: Range<T>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}
