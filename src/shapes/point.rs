use crate::shapes::RectDirection;
use derive_more::{Add, Sub};
use rkyv::{Archive, Deserialize, Serialize};

pub use vlsir::raw::Point as RawPoint;

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
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl From<ArchivedPoint> for Point {
    fn from(p: ArchivedPoint) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl PartialEq<Self> for ArchivedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for ArchivedPoint {}

impl PointLike for ArchivedPoint {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn new(x: i32, y: i32) -> Self {
        ArchivedPoint { x, y }
    }
}

pub trait PointLike: Into<Point> + Copy + Clone + PartialEq + Eq {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn new(x: i32, y: i32) -> Self;

    fn from_other<Q: PointLike>(other: &Q) -> Self {
        Self::new(other.x(), other.y())
    }

    fn from_raw(raw_point: &RawPoint) -> Self {
        Point::new(
            raw_point.x.try_into().unwrap(),
            raw_point.y.try_into().unwrap(),
        )
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
}

#[inline]
pub fn into_points<P: PointLike>(point_likes: Vec<P>) -> Vec<Point> {
    point_likes.into_iter().map(|p| p.into()).collect()
}
