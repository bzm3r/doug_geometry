use crate::shapes::{Point, PointLike, Polygon, RectDirection};
use rkyv::{Archive, Deserialize, Serialize};

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
pub struct Path {
    pub points: Vec<Point>,
    pub width: u32,
    pub layer: u8,
}

pub fn shift_pure_right<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    forward_poly_points.push(P::new(p0.x(), p0.y() - half_width));
    backward_poly_points.push(P::new(p0.x(), p0.y() + half_width));
}

pub fn shift_pure_left<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    shift_pure_right(backward_poly_points, forward_poly_points, p0, half_width)
}

pub fn shift_pure_up<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    forward_poly_points.push(P::new(p0.x() + half_width, p0.y()));
    backward_poly_points.push(P::new(p0.x() - half_width, p0.y()));
}

pub fn shift_pure_down<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    shift_pure_up(backward_poly_points, forward_poly_points, p0, half_width)
}

pub fn shift_right_up<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    // println!("shift right up");
    forward_poly_points.push(P::new(p0.x() + half_width, p0.y() - half_width));
    backward_poly_points.push(P::new(p0.x() - half_width, p0.y() + half_width));
}

pub fn shift_left_down<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    // println!("shift left down (calling shift right up)");
    shift_right_up(backward_poly_points, forward_poly_points, p0, half_width);
}

pub fn shift_right_down<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    // println!("shift right down");
    forward_poly_points.push(P::new(p0.x() - half_width, p0.y() - half_width));
    backward_poly_points.push(P::new(p0.x() + half_width, p0.y() + half_width));
}

pub fn shift_left_up<P: PointLike>(
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
    p0: P,
    half_width: i32,
) {
    // println!("shift left up (calling shift right down)");
    shift_right_down(backward_poly_points, forward_poly_points, p0, half_width);
}

/// A 2-point path that moves horizontally
pub fn simple_horizontal_path_to_poly<P: PointLike>(
    points: &[P],
    width: u32,
    layer: u8,
) -> Polygon {
    let half_width = (width / 2) as i32;

    let points = vec![
        Point::new(points[0].x(), points[0].y() - half_width),
        Point::new(points[1].x(), points[1].y() - half_width),
        Point::new(points[1].x(), points[1].y() + half_width),
        Point::new(points[0].x(), points[0].y() + half_width),
    ];

    Polygon { points, layer }
}

/// A 2-point path that moves vertically
pub fn simple_vertical_path_to_poly<P: PointLike>(points: &[P], width: u32, layer: u8) -> Polygon {
    let half_width = (width / 2) as i32;

    let points = vec![
        Point::new(points[0].x() + half_width, points[0].y()),
        Point::new(points[1].x() + half_width, points[1].y()),
        Point::new(points[1].x() - half_width, points[1].y()),
        Point::new(points[0].x() - half_width, points[0].y()),
    ];

    Polygon { points, layer }
}

pub fn start_or_end_path_to_poly<P: PointLike>(
    start_or_end_point: P,
    start_or_end_direction: RectDirection,
    half_width: i32,
    forward_poly_points: &mut Vec<P>,
    backward_poly_points: &mut Vec<P>,
) {
    match start_or_end_direction {
        RectDirection::Right => shift_pure_right(
            forward_poly_points,
            backward_poly_points,
            start_or_end_point,
            half_width,
        ),
        RectDirection::Left => shift_pure_left(
            forward_poly_points,
            backward_poly_points,
            start_or_end_point,
            half_width,
        ),
        RectDirection::Up => shift_pure_up(
            forward_poly_points,
            backward_poly_points,
            start_or_end_point,
            half_width,
        ),
        RectDirection::Down => shift_pure_down(
            forward_poly_points,
            backward_poly_points,
            start_or_end_point,
            half_width,
        ),
    }
}

pub fn path_to_poly<P: PointLike>(points: &[P], width: u32, layer: u8) -> Polygon {
    let num_points = points.len();

    // TODO: remove this assert because all shape verification should happen prior to archiving
    assert_eq!(
        width % 2,
        0,
        "width must be even for our code's assumptions to hold!"
    );

    if num_points == 2 {
        return if points[0].x() == points[1].x() {
            simple_horizontal_path_to_poly(points, width, layer)
        } else {
            simple_vertical_path_to_poly(points, width, layer)
        };
    }

    let half_width = (width / 2) as i32; // assuming that widths are even!

    let mut forward_poly_points = Vec::with_capacity(num_points);
    let mut backward_poly_points = Vec::with_capacity(num_points);

    // TODO: remove this assert because all shape verification should happen prior to archiving
    assert!(
        num_points > 1,
        "Expected number of points in path to be > 1"
    );

    let start_direction = points[0].simple_directions_to(&points[1]);

    start_or_end_path_to_poly(
        points[0],
        start_direction,
        half_width,
        &mut forward_poly_points,
        &mut backward_poly_points,
    );

    let mut last_direction = start_direction;

    for ix in 1..(num_points - 1) {
        let p0 = points[ix];
        let p1 = points[ix + 1];
        let next_move = p0.simple_directions_to(&p1);

        match (last_direction, next_move) {
            (RectDirection::Right, RectDirection::Right) => shift_pure_right(
                &mut forward_poly_points,
                &mut backward_poly_points,
                p0,
                half_width,
            ),
            (RectDirection::Left, RectDirection::Left) => shift_pure_left(
                &mut forward_poly_points,
                &mut backward_poly_points,
                p0,
                half_width,
            ),
            (RectDirection::Up, RectDirection::Up) => shift_pure_up(
                &mut forward_poly_points,
                &mut backward_poly_points,
                p0,
                half_width,
            ),
            (RectDirection::Down, RectDirection::Down) => shift_pure_down(
                &mut forward_poly_points,
                &mut backward_poly_points,
                p0,
                half_width,
            ),
            (RectDirection::Right, RectDirection::Down)
            | (RectDirection::Down, RectDirection::Right) => shift_right_down(
                &mut forward_poly_points,
                &mut backward_poly_points,
                p0,
                half_width,
            ),
            (RectDirection::Right, RectDirection::Up)
            | (RectDirection::Up, RectDirection::Right) => shift_right_up(
                &mut forward_poly_points,
                &mut backward_poly_points,
                p0,
                half_width,
            ),
            (RectDirection::Left, RectDirection::Up) | (RectDirection::Up, RectDirection::Left) => {
                shift_left_up(
                    &mut forward_poly_points,
                    &mut backward_poly_points,
                    p0,
                    half_width,
                )
            }
            (RectDirection::Left, RectDirection::Down)
            | (RectDirection::Down, RectDirection::Left) => shift_left_down(
                &mut forward_poly_points,
                &mut backward_poly_points,
                p0,
                half_width,
            ),
            (_, _) => panic!(
                "Received opposing last/next moves!" // "last: {last_move:?}, next: {next_move:?}"
            ),
        }
        last_direction = next_move;
    }

    let end_direction = points[num_points - 2].simple_directions_to(&points[num_points - 1]);

    start_or_end_path_to_poly(
        points[num_points - 1],
        end_direction,
        half_width,
        &mut forward_poly_points,
        &mut backward_poly_points,
    );

    let points: Vec<Point> = forward_poly_points
        .into_iter()
        .chain(backward_poly_points.into_iter().rev())
        .map(|p| p.into())
        .collect();

    Polygon { points, layer }
}

impl Path {
    pub fn as_poly(&self) -> Polygon {
        path_to_poly(&self.points, self.width, self.layer)
    }
}

impl ArchivedPath {
    pub fn as_poly(&self) -> Polygon {
        path_to_poly(&self.points, self.width, self.layer)
    }
}
