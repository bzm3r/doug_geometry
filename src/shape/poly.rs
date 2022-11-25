use crate::bbox::{BoundingBox, CalculateBoundingBox, UnvalidatedBoundingBox};
use crate::shape::Point;
use rkyv::{vec::ArchivedVec, Archive, Deserialize, Serialize};
use std::ops::{Add, Sub};

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
