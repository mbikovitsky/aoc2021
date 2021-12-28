use std::ops::{Add, Range, Sub};

use itertools::iproduct;
use num::{CheckedSub, Integer, Num};

pub trait CSGObject {
    fn bounding_box(&self) -> AABox;

    fn contains(&self, point: &Point) -> bool;

    fn volume(&self) -> u64;

    fn intersection_volume(&self, rhs: &dyn CSGObject) -> u64 {
        let bb_intersection = self.bounding_box().intersection(&rhs.bounding_box());

        bb_intersection
            .points()
            .filter(|point| self.contains(point) && rhs.contains(point))
            .count()
            .try_into()
            .unwrap()
    }
}

impl Add<Box<dyn CSGObject>> for Box<dyn CSGObject> {
    type Output = Box<dyn CSGObject>;

    fn add(self, rhs: Box<dyn CSGObject>) -> Self::Output {
        Box::new(Union {
            left: self,
            right: rhs,
        })
    }
}

impl Sub<Box<dyn CSGObject>> for Box<dyn CSGObject> {
    type Output = Box<dyn CSGObject>;

    fn sub(self, rhs: Box<dyn CSGObject>) -> Self::Output {
        Box::new(Difference {
            left: self,
            right: rhs,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nothing;

impl CSGObject for Nothing {
    fn bounding_box(&self) -> AABox {
        AABox {
            x: 0..0,
            y: 0..0,
            z: 0..0,
        }
    }

    fn contains(&self, _point: &Point) -> bool {
        false
    }

    fn volume(&self) -> u64 {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CSGObject for Point {
    fn bounding_box(&self) -> AABox {
        AABox {
            x: self.x..self.x + 1,
            y: self.y..self.y + 1,
            z: self.z..self.z + 1,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        self == point
    }

    fn volume(&self) -> u64 {
        1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AABox {
    pub x: Range<i32>,
    pub y: Range<i32>,
    pub z: Range<i32>,
}

impl AABox {
    pub fn intersection(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.intersection(&rhs.x),
            y: self.y.intersection(&rhs.y),
            z: self.z.intersection(&rhs.z),
        }
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        iproduct!(self.x.clone(), self.y.clone(), self.z.clone()).map(|(x, y, z)| Point { x, y, z })
    }
}

impl CSGObject for AABox {
    fn bounding_box(&self) -> AABox {
        self.clone()
    }

    fn contains(&self, point: &Point) -> bool {
        self.x.contains(&point.x) && self.y.contains(&point.y) && self.z.contains(&point.z)
    }

    fn volume(&self) -> u64 {
        let x_length: u64 = self.x.len().try_into().unwrap();
        let y_length: u64 = self.y.len().try_into().unwrap();
        let z_length: u64 = self.z.len().try_into().unwrap();

        x_length * y_length * z_length
    }
}

pub struct Union {
    pub left: Box<dyn CSGObject>,
    pub right: Box<dyn CSGObject>,
}

impl CSGObject for Union {
    fn bounding_box(&self) -> AABox {
        let left_bb = self.left.bounding_box();
        let right_bb = self.right.bounding_box();
        AABox {
            x: bounding_interval(&left_bb.x, &right_bb.x),
            y: bounding_interval(&left_bb.y, &right_bb.y),
            z: bounding_interval(&left_bb.z, &right_bb.z),
        }
    }

    fn contains(&self, point: &Point) -> bool {
        self.left.contains(point) || self.right.contains(point)
    }

    fn volume(&self) -> u64 {
        self.left
            .volume()
            .checked_add(self.right.volume())
            .unwrap()
            .checked_sub(self.left.intersection_volume(self.right.as_ref()))
            .unwrap()
    }
}

fn bounding_interval<T>(a: &Range<T>, b: &Range<T>) -> Range<T>
where
    T: Integer + Clone,
{
    std::cmp::min(&a.start, &b.start).clone()..std::cmp::max(&a.end, &b.end).clone()
}

pub struct Difference {
    pub left: Box<dyn CSGObject>,
    pub right: Box<dyn CSGObject>,
}

impl CSGObject for Difference {
    fn bounding_box(&self) -> AABox {
        // ¯\_(ツ)_/¯
        self.left.bounding_box()
    }

    fn contains(&self, point: &Point) -> bool {
        let inside_left = self.left.contains(point);
        let inside_right = self.right.contains(point);

        if inside_left && inside_right {
            false
        } else {
            inside_left
        }
    }

    fn volume(&self) -> u64 {
        self.left.volume() - self.left.intersection_volume(self.right.as_ref())
    }
}

trait NumRangeExt<T: Num> {
    fn intersection(&self, rhs: &Self) -> Self;
}

impl<T> NumRangeExt<T> for Range<T>
where
    T: Integer + CheckedSub + Clone,
{
    fn intersection(&self, rhs: &Self) -> Self {
        let start = std::cmp::max(&self.start, &rhs.start);
        let end = std::cmp::min(&self.end, &rhs.end);
        Self {
            start: start.clone(),
            end: end.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AABox, CSGObject};

    #[test]
    fn sanity() {
        let cube1 = Box::new(AABox {
            x: 10..13,
            y: 10..13,
            z: 10..13,
        }) as Box<dyn CSGObject>;

        let cube2 = Box::new(AABox {
            x: 11..14,
            y: 11..14,
            z: 11..14,
        }) as Box<dyn CSGObject>;

        let cube3 = Box::new(AABox {
            x: 9..12,
            y: 9..12,
            z: 9..12,
        }) as Box<dyn CSGObject>;

        let cube4 = Box::new(AABox {
            x: 10..11,
            y: 10..11,
            z: 10..11,
        }) as Box<dyn CSGObject>;

        let result = cube1 + cube2 - cube3 + cube4;

        assert_eq!(result.volume(), 39);
    }
}
