use crate::shapes::Point;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(
    Debug,
    Eq,
    PartialEq,
    Archive,
    Deserialize,
    Serialize,
    Clone,
    Hash,
    Copy,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Rect {
    pub p0: Point,
    pub p1: Point,
    pub layer: u8,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RectDirection {
    Left,
    Right,
    Up,
    Down,
}
