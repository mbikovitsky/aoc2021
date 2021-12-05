use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let coordinates: Result<Vec<u32>> = string
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

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn points(&self) -> Box<dyn Iterator<Item = Point>> {
        if self.horizontal() {
            let y = self.start.y;

            if self.start.x <= self.end.x {
                Box::new((self.start.x..=self.end.x).map(move |x| Point { x, y }))
            } else {
                Box::new(
                    (self.end.x..=self.start.x)
                        .rev()
                        .map(move |x| Point { x, y }),
                )
            }
        } else if self.vertical() {
            let x = self.start.x;

            if self.start.y <= self.end.y {
                Box::new((self.start.y..=self.end.y).map(move |y| Point { x, y }))
            } else {
                Box::new(
                    (self.end.y..=self.start.y)
                        .rev()
                        .map(move |y| Point { x, y }),
                )
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
