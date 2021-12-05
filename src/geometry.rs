use std::{
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl AsRef<Vector> for Vector {
    fn as_ref(&self) -> &Vector {
        self
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let coordinates: Result<Vec<i32>> = string
            .split(',')
            .map(|coordinate| Ok(coordinate.parse()?))
            .collect();
        let coordinates = coordinates?;

        assert_eq!(coordinates.len(), 2);

        let x = coordinates[0];
        let y = coordinates[1];

        Ok(Point { x, y })
    }
}

impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Point {
        self
    }
}

impl Sub<Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_diagonal(&self) -> bool {
        let delta = self.end - self.start;
        delta.x.unsigned_abs() == delta.y.unsigned_abs()
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        if self.is_horizontal() {
            if self.start.x <= self.end.x {
                LinePointsIterator {
                    current: Some(self.start),
                    step: Vector { x: 1, y: 0 },
                    end: self.end,
                }
            } else {
                LinePointsIterator {
                    current: Some(self.start),
                    step: Vector { x: -1, y: 0 },
                    end: self.end,
                }
            }
        } else if self.is_vertical() {
            if self.start.y <= self.end.y {
                LinePointsIterator {
                    current: Some(self.start),
                    step: Vector { x: 0, y: 1 },
                    end: self.end,
                }
            } else {
                LinePointsIterator {
                    current: Some(self.start),
                    step: Vector { x: 0, y: -1 },
                    end: self.end,
                }
            }
        } else {
            todo!("Only horizontal and vertical lines are implemented")
        }
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let points: Result<Vec<Point>> = string
            .split("->")
            .map(|coordinates| Ok(coordinates.trim().parse()?))
            .collect();
        let points = points?;

        assert_eq!(points.len(), 2);

        let start = points[0];
        let end = points[1];

        Ok(Line { start, end })
    }
}

impl AsRef<Line> for Line {
    fn as_ref(&self) -> &Line {
        self
    }
}

struct LinePointsIterator {
    current: Option<Point>,
    step: Vector,
    end: Point,
}

impl Iterator for LinePointsIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.current = if current == self.end {
                None
            } else {
                Some(current + self.step)
            };

            Some(current)
        } else {
            None
        }
    }
}
