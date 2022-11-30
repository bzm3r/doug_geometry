mod bbox;
pub mod shapes;
pub mod decomp;

use crate::shapes::{PointLike, PolyRect, Polygon, RectDirection};
//
// impl PolyDecomposer {
//     pub fn from_poly(poly: Polygon) -> PolyDecomposer {
//         let mut points = poly.points.clone();
//         // TODO: remove this assert because all shape verification should happen prior to archiving
//         assert!(points.len() > 3);
//
//         let mut directions = Vec::<RectDirection>::with_capacity(points.len() - 1);
//         let mut corners = Vec::<CornerType>::with_capacity(points.len());
//         corners.push(CornerType::DownRight);
//
//         use RectDirection::*;
//         let mut last_horizontal_ix: Option<usize> = None;
//         let mut last_vertical_ix: Option<usize> = None;
//
//         let mut vertical_walls = Walls::new(WallType::Vertical {
//             attitude: RectDirection::Up,
//         });
//         let mut current_vertical_wall = Wall::with_capacity(points.len() / 2);
//         let mut horizontal_walls = Walls::new(WallType::Horizontal {
//             attitude: RectDirection::Right,
//         });
//         let mut current_horizontal_wall = Wall::with_capacity(points.len() / 2);
//
//         for point_window in points.as_slice().windows(2) {
//             let [p0, p1]: [_; 2] = point_window.try_into().ok().unwrap();
//
//             let direction = p0.simple_directions_to(&p1);
//
//             if !directions.is_empty() {
//                 match direction {
//                     Left | Right => {
//                         if let Some(ix) = last_horizontal_ix {
//                             if directions[ix] != direction {
//                                 horizontal_walls.push(current_horizontal_wall.clone());
//                                 current_horizontal_wall.clear();
//                             }
//                         }
//
//                         directions.push(direction);
//                         last_horizontal_ix.replace(directions.len() - 1);
//                     }
//                     Up | Down => {
//                         if let Some(ix) = last_vertical_ix {
//                             if directions[ix] != direction {
//                                 vertical_inversions += 1;
//                             }
//                         }
//                         directions.push(direction);
//                         last_vertical_ix.replace(directions.len() - 1);
//                     }
//                 }
//             } else {
//                 directions.push(direction)
//             }
//         }
//
//         PolyDecomposer { points, directions }
//     }
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Copy)]
// pub struct LineSeg {
//     p0: Point,
//     p1: Point,
// }
