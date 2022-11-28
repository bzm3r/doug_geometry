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
    pub rect: Rectangle,
    pub layer: u8,
}

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
pub struct Rectangle {
    pub p0: Point,
    pub p1: Point,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    Archive,
    Deserialize,
    Serialize,
    Clone,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct PolyRect {
    pub rects: Vec<Rectangle>,
    pub layer: u8,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RectDirection {
    Down,
    Left,
    Right,
    Up,
}

impl RectDirection {
    pub fn is_horizontal(&self) -> bool {
        match self {
            RectDirection::Right | RectDirection::Left => true,
            _ => false,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            RectDirection::Up | RectDirection::Down => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum VerticalDirection {
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum HorizontalDirection {
    Left,
    Right,
}

impl From<VerticalDirection> for RectDirection {
    fn from(value: VerticalDirection) -> Self {
        match value {
            VerticalDirection::Up => RectDirection::Up,
            VerticalDirection::Down => RectDirection::Down,
        }
    }
}

impl From<HorizontalDirection> for RectDirection {
    fn from(value: HorizontalDirection) -> Self {
        match value {
            HorizontalDirection::Left => RectDirection::Left,
            HorizontalDirection::Right => RectDirection::Right,
        }
    }
}
