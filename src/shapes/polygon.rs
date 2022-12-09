use crate::decomp::{Corners, RectCorner};
use crate::shapes::Point;
use rkyv::{Archive, Deserialize, Serialize};
use vlsir::raw::Point as RawPoint;

#[derive(
    Debug,
    Eq,
    PartialEq,
    Hash,
    Archive,
    Deserialize,
    Serialize,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Polygon {
    pub points: Vec<Point>,
    pub layer: u8,
}

impl Polygon {
    pub fn from_raw(_raw_points: Vec<RawPoint>) -> Polygon {
        todo!()
    }

    pub fn corners(&self) -> Corners<Point> {
        let mut corners = Vec::with_capacity(self.points.len());

        for ix in 0..self.points.len() {
            let last_ix = (ix - 1) % self.points.len();
            let next_ix = (ix + 1) % self.points.len();
            let incoming = self.points[last_ix];
            let outgoing = self.points[next_ix];
            let center = self.points[ix];
            corners.push(RectCorner::new(incoming, outgoing, center));
        }

        Corners::new(corners)
    }
}
