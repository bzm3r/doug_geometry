use crate::shapes::{PointLike, RectDirection};
use std::collections::HashSet;

pub fn deduplicate_points<P: PointLike>(points: Vec<P>) -> Vec<P> {
    let mut encountered_points = HashSet::with_capacity(points.len());
    let mut deduplicated_points = Vec::with_capacity(points.len());

    for point in points {
        if !encountered_points.contains(&point) {
            encountered_points.insert(point);
            deduplicated_points.push(point);
        }
    }

    deduplicated_points.shrink_to_fit();
    deduplicated_points
}

/// Checks to see if points are are all placed horizontally/vertically with respect to each other,
/// and gets the directions between each point sequence.
pub fn check_sequence_rectilinearity<P: PointLike>(points: &[P]) -> Vec<RectDirection> {
    points
        .windows(2)
        .map(|point_window| {
            let [p0, p1]: [_; 2] = point_window.try_into().ok().unwrap();
            let directions_to = p0.directions_to(p1);
            assert_eq!(directions_to.len(), 1);
            directions_to[0]
        })
        .collect()
}
