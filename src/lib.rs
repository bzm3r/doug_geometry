use derive_more::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Add, Sub)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RectDirections {
    Left,
    Right,
    Up,
    Down,
}

impl Point {
    fn directions_to(&self, other: &Point) -> Vec<RectDirections> {
        let mut result = Vec::with_capacity(2);
        use RectDirections::*;
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
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Rect {
    pub p0: Point,
    pub p1: Point,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct LayerRect {
    pub rect: Rect,
    pub layer: u8,
}

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

        let directions = Vec::<RectDirections>::with_capacity(points.len() - 1);

        use RectDirections::*;
        let mut reverse_points = false;
        let mut bottom_left_most = 0;
        let mut last_horizontal_ix: Option<usize> = None;
        let mut last_vertical_ix: Option<usize> = None;
        let mut horizontal_inversions: usize = 0;
        let mut vertical_inversions: usize = 0;

        for [p0, p1] in points.as_slice().windows(2) {
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

impl RectilinearPolygon {
    pub fn count_inversions(&self) -> usize {
        let mut num_horizontal_inversions = 0;
        let mut num_vertical_inversion = 0;

        for (ix, point) in self.points.iter().enumerate() {
            if ix > 0 {}
        }
    }
}
