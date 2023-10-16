use std::cmp::Ordering;

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

    fn directions_to<Q: PointLike>(&self, other: &Q) -> Vec<RectDirection> {
        let mut result = Vec::with_capacity(2);
        use RectDirection::*;

        match self.x().cmp(&other.x()) {
            Ordering::Less => result.push(Right),
            Ordering::Equal => {}
            Ordering::Greater => result.push(Left),
        }

        match self.y().cmp(&other.y()) {
            Ordering::Less => result.push(Up),
            Ordering::Equal => {}
            Ordering::Greater => result.push(Down),
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
