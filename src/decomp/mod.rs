mod rect_corner;
mod wall;
mod wall_model;

pub use rect_corner::*;
pub use wall::*;
pub use wall_model::*;

pub mod PolyBuilder {}

#[cfg(test)]
mod tests {
    use crate::shapes::{path_to_poly, Path, Point, PointLike, Polygon};

    fn spiral_path(vertical_inversions: usize) -> Path {
        let mut vertical = true;
        let mut executed_inversions = 0;
        let mut points = Vec::with_capacity(2 * vertical_inversions);
        let mut jump = 0;
        let mut cursor_point = Point::new(0, 0);

        points.push(cursor_point);
        while executed_inversions <= vertical_inversions {
            if vertical {
                jump += 1;
                cursor_point = cursor_point + Point::new(0, jump);
                executed_inversions += 1;
            }
            {
                jump += 2;
                cursor_point = cursor_point + Point::new(jump, 0);
            }

            points.push(cursor_point);
            vertical = !vertical;
        }

        Path {
            points,
            width: 2,
            layer: 0,
        }
    }

    fn spiral_poly(vertical_inversions: usize) -> Polygon {
        let Path {
            points,
            width,
            layer,
        } = spiral_path(vertical_inversions);
        path_to_poly(&points, width, layer)
    }

    fn rotate_poly(polygon: Polygon) -> Polygon {
        let Polygon { points, layer } = polygon;
    }
}
