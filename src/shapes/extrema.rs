use crate::shapes::PointLike;

/// Finds the point (`a`, `b`) such that:
/// * `a` is the maximum x-value attained by any point in `point`
/// * `b` is the minimum y-value attained by all points with x-value `a`
pub fn find_max_x_min_y_point<P: PointLike>(points: &[P]) -> usize {
    let max_x = points.map(|point| point.x).max().unwrap();

    let min_y_of_max_x = points
        .filter_map(|point| (point.x == max_x).then_some(point.y))
        .min()
        .unwrap();

    let max_x_min_y_point = P::new(max_x, min_y_of_max_x);

    points
        .enumerate()
        .find_map(|(ix, point)| (point == max_x_min_y_point).then_some(ix))
        .unwrap()
}
