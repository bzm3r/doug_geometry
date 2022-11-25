use crate::bbox::{BoundingBox, CalculateBoundingBox, UnvalidatedBoundingBox};
use rkyv::{vec::ArchivedVec, Archive, Deserialize, Serialize};

mod path;
mod point;
mod poly;
mod rect;

pub use path::*;
pub use point::*;
pub use poly::*;
pub use rect::*;

#[derive(
    Debug, Eq, PartialEq, Archive, Deserialize, Serialize, serde::Serialize, serde::Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Shapes {
    pub bbox: BoundingBox,
    pub shapes: Vec<Shape>,
}

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
pub enum Shape {
    Rect(Rect),
    Poly(Poly),
    Path(Path),
}

impl CalculateBoundingBox for ArchivedShape {
    fn bbox(&self) -> BoundingBox {
        match self {
            Self::Rect(r) => {
                let mut bbox = UnvalidatedBoundingBox::invalid();

                bbox.min.x = r.p0.x.min(bbox.min.x);
                bbox.min.y = r.p0.y.min(bbox.min.y);
                bbox.max.x = r.p0.x.max(bbox.max.x);
                bbox.max.y = r.p0.y.max(bbox.max.y);
                bbox.min.x = r.p1.x.min(bbox.min.x);
                bbox.min.y = r.p1.y.min(bbox.min.y);
                bbox.max.x = r.p1.x.max(bbox.max.x);
                bbox.max.y = r.p1.y.max(bbox.max.y);

                BoundingBox::new(bbox)
            }
            Self::Poly(p) => {
                let mut bbox = UnvalidatedBoundingBox::invalid();
                for pt in p.points.iter() {
                    bbox.min.x = pt.x.min(bbox.min.x);
                    bbox.min.y = pt.y.min(bbox.min.y);
                    bbox.max.x = pt.x.max(bbox.max.x);
                    bbox.max.y = pt.y.max(bbox.max.y);
                }
                BoundingBox::new(bbox)
            }
            Self::Path(p) => {
                let mut bbox = UnvalidatedBoundingBox::invalid();

                let p = p.as_poly();

                for pt in p.points.iter() {
                    bbox.min.x = pt.x.min(bbox.min.x);
                    bbox.min.y = pt.y.min(bbox.min.y);
                    bbox.max.x = pt.x.max(bbox.max.x);
                    bbox.max.y = pt.y.max(bbox.max.y);
                }
                BoundingBox::new(bbox)
            }
        }
    }
}

impl CalculateBoundingBox for Shape {
    fn bbox(&self) -> BoundingBox {
        match self {
            Self::Rect(r) => {
                let mut bbox = UnvalidatedBoundingBox::invalid();

                bbox.min.x = r.p0.x.min(bbox.min.x);
                bbox.min.y = r.p0.y.min(bbox.min.y);
                bbox.max.x = r.p0.x.max(bbox.max.x);
                bbox.max.y = r.p0.y.max(bbox.max.y);
                bbox.min.x = r.p1.x.min(bbox.min.x);
                bbox.min.y = r.p1.y.min(bbox.min.y);
                bbox.max.x = r.p1.x.max(bbox.max.x);
                bbox.max.y = r.p1.y.max(bbox.max.y);

                BoundingBox::new(bbox)
            }
            Self::Poly(p) => {
                let mut bbox = UnvalidatedBoundingBox::invalid();
                for pt in p.points.iter() {
                    bbox.min.x = pt.x.min(bbox.min.x);
                    bbox.min.y = pt.y.min(bbox.min.y);
                    bbox.max.x = pt.x.max(bbox.max.x);
                    bbox.max.y = pt.y.max(bbox.max.y);
                }
                BoundingBox::new(bbox)
            }
            Self::Path(p) => {
                let mut bbox = UnvalidatedBoundingBox::invalid();

                let p = p.as_poly();

                for pt in p.points.iter() {
                    bbox.min.x = pt.x.min(bbox.min.x);
                    bbox.min.y = pt.y.min(bbox.min.y);
                    bbox.max.x = pt.x.max(bbox.max.x);
                    bbox.max.y = pt.y.max(bbox.max.y);
                }
                BoundingBox::new(bbox)
            }
        }
    }
}

pub fn bbbox(arc: std::sync::Arc<&ArchivedVec<ArchivedShape>>) -> BoundingBox {
    let len = arc.len();
    // let cpus = std::thread::available_parallelism().unwrap().get();
    let cpus = 12;

    let rem = len % cpus;

    println!("len: {}, num_cpus: {}, {}", len, cpus, rem);

    let mut bbox = UnvalidatedBoundingBox::invalid();

    for x in (len - rem)..len {
        println!("{x}");

        bbox.update(&arc[x])
    }

    std::thread::scope(|s| {
        let mut bboxes = vec![];

        for t in 0..cpus {
            let arc = arc.clone();
            bboxes.push(s.spawn(move || {
                let mut bbox = UnvalidatedBoundingBox::invalid();

                for x in (t..len).step_by(cpus) {
                    bbox.update(&arc[x]);
                    // if x > (len - 100) {
                    //     println!("{t}: {x} {:?}", arc[x]);
                    // }
                }
                bbox
            }));
        }

        for bb in bboxes {
            bbox.update_bbox(&bb.join().unwrap());
        }
    });

    BoundingBox::new(bbox)
}

// impl CalculateBoundingBox for std::sync::Arc<&ArchivedVec<ArchivedShape>> {
//     fn bbox(&self) -> BoundingBox {
//         let len = self.len();
//         let cpus = std::thread::available_parallelism().unwrap().get();
//         let rem = len % cpus;

//         let mut bbox = UnvalidatedBoundingBox::invalid();

//         for x in (len - rem)..len {
//             println!("{x}");

//             bbox.update(&self[x])
//         }

//         for t in 0..cpus {
//             let reader = self.clone();

//             std::thread::spawn(move || {
//                 let len = len.clone();
//                 let cpus = cpus.clone();

//                 for x in (t..len).step_by(cpus) {
//                     if x > (len - 100) {
//                         println!("{t}: {x} {:?}", reader[x]);
//                     }
//                 }
//             });
//         }

//         println!("len: {}, num_cpus: {}, {}", len, cpus, rem);
//         let mut bbox = UnvalidatedBoundingBox::invalid();
//         for s in self.iter() {
//             bbox.update(s);
//         }
//         BoundingBox::new(bbox)
//     }
// }

impl CalculateBoundingBox for Vec<Shape> {
    fn bbox(&self) -> BoundingBox {
        let mut shapes = self.iter();
        let mut bbox = shapes.next().unwrap().bbox();
        for s in shapes {
            bbox.union(&s.bbox());
        }
        bbox
    }
}
