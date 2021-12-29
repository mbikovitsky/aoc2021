use num::{CheckedMul, CheckedSub, Integer};

use crate::int_interval::IntInterval;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Box<T: Integer> {
    pub x: IntInterval<T>,
    pub y: IntInterval<T>,
    pub z: IntInterval<T>,
}

impl<T: Integer> Box<T> {
    pub fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }
}

impl<T: Integer + CheckedSub + CheckedMul> Box<T> {
    pub fn volume(&self) -> Option<T> {
        match (self.x.len(), self.y.len(), self.z.len()) {
            (Some(x), Some(y), Some(z)) => {
                if let Some(xy) = x.checked_mul(&y) {
                    xy.checked_mul(&z)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl<T: Integer + Clone> Box<T> {
    pub fn intersect(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.intersect(&rhs.x),
            y: self.y.intersect(&rhs.y),
            z: self.z.intersect(&rhs.z),
        }
    }

    pub fn subtract_split(&self, rhs: &Self) -> impl Iterator<Item = Self> {
        let intersection = self.intersect(rhs);

        let boxen = [
            Self {
                x: (self.x.start.clone()..intersection.x.start.clone()).into(),
                y: self.y.clone(),
                z: self.z.clone(),
            }
            .intersect(&self),
            Self {
                x: (intersection.x.end.clone()..self.x.end.clone()).into(),
                y: self.y.clone(),
                z: self.z.clone(),
            }
            .intersect(&self),
            Self {
                x: intersection.x.clone(),
                y: (self.y.start.clone()..intersection.y.start.clone()).into(),
                z: self.z.clone(),
            }
            .intersect(&self),
            Self {
                x: intersection.x.clone(),
                y: (intersection.y.end.clone()..self.y.end.clone()).into(),
                z: self.z.clone(),
            }
            .intersect(&self),
            Self {
                x: intersection.x.clone(),
                y: intersection.y.clone(),
                z: (self.z.start.clone()..intersection.z.start.clone()).into(),
            }
            .intersect(self),
            Self {
                x: intersection.x.clone(),
                y: intersection.y.clone(),
                z: (intersection.z.end.clone()..self.z.end.clone()).into(),
            }
            .intersect(self),
        ];

        boxen.into_iter().filter(|sub_box| !sub_box.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::Box;

    #[test]
    fn rubik_center() {
        let cube = Box {
            x: (0..3).into(),
            y: (0..3).into(),
            z: (0..3).into(),
        };

        let center = Box {
            x: (1..2).into(),
            y: (1..2).into(),
            z: (1..2).into(),
        };

        let slices: Vec<_> = cube.subtract_split(&center).collect();
        let total_volume: i32 = slices.iter().map(|slice| slice.volume().unwrap()).sum();
        assert_eq!(total_volume, 27 - 1);
    }

    #[test]
    fn rubik_corner() {
        let cube = Box {
            x: (0..3).into(),
            y: (0..3).into(),
            z: (0..3).into(),
        };

        let corner = Box {
            x: (2..3).into(),
            y: (2..3).into(),
            z: (2..3).into(),
        };

        let slices: Vec<_> = cube.subtract_split(&corner).collect();
        let total_volume: i32 = slices.iter().map(|slice| slice.volume().unwrap()).sum();
        assert_eq!(total_volume, 27 - 1);
    }

    #[test]
    fn rubik_bar() {
        let cube = Box {
            x: (0..3).into(),
            y: (0..3).into(),
            z: (0..3).into(),
        };

        let bar = Box {
            x: (0..3).into(),
            y: (1..2).into(),
            z: (1..2).into(),
        };

        let slices: Vec<_> = cube.subtract_split(&bar).collect();
        let total_volume: i32 = slices.iter().map(|slice| slice.volume().unwrap()).sum();
        assert_eq!(total_volume, 27 - 3);
    }
}
