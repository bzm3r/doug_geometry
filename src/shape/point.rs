use crate::bbox::{BoundingBox, CalculateBoundingBox, UnvalidatedBoundingBox};
use crate::shape::RectDirection;
use rkyv::{vec::ArchivedVec, Archive, Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(
    Debug,
    Eq,
    PartialEq,
    Archive,
    Default,
    Deserialize,
    Serialize,
    Clone,
    Hash,
    Copy,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, Copy, Clone))]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn directions_to(&self, other: &Point) -> Vec<RectDirection> {
        let mut result = Vec::with_capacity(2);
        use RectDirection::*;
        if self.x > other.x {
            result.push(Left)
        } else if self.x < other.x {
            result.push(Right)
        }

        if self.y > other.y {
            result.push(Down)
        } else if self.x < other.y {
            result.push(Up)
        }
        result
    }

    pub fn simple_directions_to(&self, other: &Point) -> RectDirection {
        // TODO: remove this assert because all shape verification should happen prior to archiving?
        let moves_to = self.directions_to(other);
        assert_eq!(move_to.len(), 1);
        moves_to[0]
    }
}

impl From<ArchivedPoint> for Point {
    fn from(p: ArchivedPoint) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for ArchivedPoint {
    type Output = ArchivedPoint;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&ArchivedPoint> for ArchivedPoint {
    type Output = ArchivedPoint;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &ArchivedPoint {
    type Output = ArchivedPoint;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&ArchivedPoint> for &ArchivedPoint {
    type Output = ArchivedPoint;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for ArchivedPoint {
    type Output = ArchivedPoint;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<&ArchivedPoint> for ArchivedPoint {
    type Output = ArchivedPoint;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &ArchivedPoint {
    type Output = ArchivedPoint;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<&ArchivedPoint> for &ArchivedPoint {
    type Output = ArchivedPoint;

    fn sub(self, rhs: Self::Rhs) -> Self::Output {
        ArchivedPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
