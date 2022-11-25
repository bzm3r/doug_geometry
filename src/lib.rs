mod bbox;
pub mod shape;

use crate::shape::Point;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Polygon {
    pub points: Vec<Point>,
    pub directions: Vec<RectDirections>,
    pub horizontal_inversions: usize,
    pub vertical_inversions: usize,
}

impl Polygon {
    pub fn find_bottom_left_most_point(&self) {}

    pub fn from_points(mut points: Vec<Point>) -> Polygon {
        assert!(points.len() > 3);

        let mut directions = Vec::<RectDirections>::with_capacity(points.len() - 1);

        use RectDirections::*;
        let mut reverse_points = false;
        let mut last_horizontal_ix: Option<usize> = None;
        let mut last_vertical_ix: Option<usize> = None;
        let mut horizontal_inversions: usize = 0;
        let mut vertical_inversions: usize = 0;

        for point_window in points.as_slice().windows(2) {
            let [p0, p1]: [_; 2] = point_window.try_into().ok().unwrap();

            let direction = {
                let directions_to = p0.directions_to(p1);
                assert_eq!(directions_to.len(), 1);
                directions_to[0]
            };

            if directions.len() > 0 {
                match direction {
                    Left | Right => {
                        if let Some(ix) = last_horizontal_ix {
                            if directions[ix] != direction {
                                horizontal_inversions += 1;
                            }
                        }
                        directions.push(direction);
                        last_horizontal_ix.replace(directions.len() - 1);
                    }
                    Up | Down => {
                        if let Some(ix) = last_vertical_ix {
                            if directions[ix] != direction {
                                vertical_inversions += 1;
                            }
                        }
                        directions.push(direction);
                        last_vertical_ix.replace(directions.len() - 1);
                    }
                }
            } else {
                match direction {
                    Left | Down => {
                        reverse_points = true;
                    }
                    _ => {}
                }
                directions.push(direction)
            }
        }

        if reverse_points {
            points.reverse();
            directions.reverse();
        }

        Polygon {
            points,
            directions,
            horizontal_inversions,
            vertical_inversions,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct LineSeg {
    p0: Point,
    p1: Point,
}
