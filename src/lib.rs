use derive_more::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy, Add, Sub)]
pub struct Point {
    x: i32,
    y: i32,
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

pub struct Polygon {
    pub points: Vec<Point>,
}

pub struct RectilinearPolygon {
    pub points: Vec<Point>,
    pub vertical_walls: Vec<Point>,
}

impl Polygon {
    pub fn count_inversions(&self) ->  {
        
    }
}

