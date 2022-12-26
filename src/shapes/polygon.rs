use crate::decomp::{Corners, RectCorner};
use crate::shapes::sanitization::{check_sequence_rectilinearity, deduplicate_points};
use crate::shapes::{Point, PointLike, RectDirection};
use rkyv::{Archive, Deserialize, Serialize};

use crate::shapes::extrema::find_max_x_min_y_point;
pub use vlsir::raw::Polygon as RawPolygon;

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
    pub fn sanitize_points(points: Vec<Point>) -> Vec<Point> {
        let mut points = deduplicate_points(points);
        let max_x_min_y_ix = find_max_x_min_y_point(&points);

        points.rotate_left(max_x_min_y_ix);
        let directions = check_sequence_rectilinearity(&points);

        match directions[0] {
            RectDirection::Down => panic!("Expected first point to be the max-x, min-y point, so there cannot be a point below it!"),
            RectDirection::Left => {
                // after this operation, the max_x_min_y point will be the last point
                points.reverse();
                // so we need to rotate one to the right
                points.rotate_right(1);
            },
            RectDirection::Right => panic!("Expected first point to be the max-x, min-y point, so there cannot be a point right of it!"),
            RectDirection::Up => {},
        }

        points
    }
    pub fn from_raw(layer: u8, raw_polygon: RawPolygon) -> Polygon {
        let points = raw_polygon.vertices.iter().map(Point::from_raw).collect();

        Polygon {
            points: Self::sanitize_points(points),
            layer,
        }
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
