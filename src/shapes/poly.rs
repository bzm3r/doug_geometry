use crate::shapes::Point;
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
pub struct Poly {
    pub points: Vec<Point>,
    pub layer: u8,
}
