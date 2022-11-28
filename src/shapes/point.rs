use crate::shapes::RectDirection;
use derive_more::{Add, Sub};
use rkyv::{Archive, Deserialize, Serialize};

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
    Add,
    Sub,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, Copy, Clone, Add, Sub))]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn shift(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl PointLike for Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl From<ArchivedPoint> for Point {
    fn from(p: ArchivedPoint) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl PointLike for ArchivedPoint {
    fn new(x: i32, y: i32) -> Self {
        ArchivedPoint { x, y }
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

pub trait PointLike: Into<Point> + Copy + Clone {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn new(x: i32, y: i32) -> Self;

    fn from_other<Q: PointLike>(other: &Q) -> Self {
        Self::new(other.x(), other.y())
    }

    fn directions_to<Q: PointLike>(&self, other: &Q) -> Vec<RectDirection> {
        let mut result = Vec::with_capacity(2);
        use RectDirection::*;
        if self.x() > other.x() {
            result.push(Left)
        } else if self.x() < other.x() {
            result.push(Right)
        }

        if self.y() > other.y() {
            result.push(Down)
        } else if self.x() < other.y() {
            result.push(Up)
        }
        result
    }

    fn simple_directions_to<Q: PointLike>(&self, other: &Q) -> RectDirection {
        // TODO: remove this assert because all shape verification should happen prior to archiving?
        let moves_to = self.directions_to(other);
        assert_eq!(moves_to.len(), 1);
        moves_to[0]
    }

    fn project_horizontal<Q: PointLike, R: PointLike>(&self, p0: &Q, p1: &R) -> Option<Self> {
        if p0.x() == self.x() {
            Some(Self::from_other(p0))
        } else if p1.x() == self.x() {
            Some(Self::from_other(p1))
        } else if p0.x() < self.x() && self.x() < p1.x() {
            Some(Self::new(self.x(), p0.y()))
        } else {
            None
        }
    }

    fn project_vertical<Q: PointLike, R: PointLike>(&self, p0: &Q, p1: &R) -> Option<Self> {
        if p0.y() == self.y() {
            Some(Self::from_other(p0))
        } else if p1.y() == self.y() {
            Some(Self::from_other(p1))
        } else if p0.y() < self.y() && self.y() < p1.y() {
            Some(Self::new(p0.x(), self.y()))
        } else {
            None
        }
    }
}

#[inline]
pub fn into_points<P: PointLike>(point_likes: Vec<P>) -> Vec<Point> {
    point_likes.into_iter().map(|p| p.into()).collect()
}
