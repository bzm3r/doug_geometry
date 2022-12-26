mod rect_corner;
mod wall;
mod wall_model;

pub use rect_corner::*;
pub use wall::*;
pub use wall_model::*;

pub mod tests {
    use crate::shapes::{path_to_poly, Path, Point, PointLike, Polygon, RectDirection};

    pub fn spiral_path(spiral_inversions: usize) -> Path {
        let moves = [
            RectDirection::Right,
            RectDirection::Up,
            RectDirection::Left,
            RectDirection::Down,
        ];
        let mut executed_moves: usize = 0;
        let mut points = Vec::with_capacity(2 * spiral_inversions);
        let mut jump = 0;
        let mut cursor_point = Point::new(0, 0);

        points.push(cursor_point);
        while executed_moves / 2 <= spiral_inversions {
            let delta = match moves[executed_moves % 4] {
                RectDirection::Down => {
                    jump += 1;
                    Point::new(0, -jump)
                }
                RectDirection::Left => {
                    jump += 2;
                    Point::new(-jump, 0)
                }
                RectDirection::Right => {
                    jump += 2;
                    Point::new(jump, 0)
                }
                RectDirection::Up => {
                    jump += 1;
                    Point::new(0, jump)
                }
            };

            executed_moves += 1;
            cursor_point = cursor_point + delta;

            points.push(cursor_point);
        }

        Path {
            points,
            width: 2,
            layer: 0,
        }
    }

    pub fn spiral_poly(spiral_inversions: usize) -> Polygon {
        let Path {
            points,
            width,
            layer,
        } = spiral_path(spiral_inversions);
        path_to_poly(&points, width, layer)
    }

    pub fn rotate_poly(polygon: Polygon) -> Polygon {
        let Polygon { points, layer: _ } = polygon;
        let points = Polygon::sanitize_points(points.iter().map(Point::rotate).collect());
        Polygon { points, layer: 0 }
    }
}
