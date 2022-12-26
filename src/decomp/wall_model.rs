use crate::decomp::{RectCorner, Wall};
use crate::shapes::{PointLike, PolyRect, Polygon, RectDirection};
use std::collections::BinaryHeap;

pub struct WallModel<P: PointLike> {
    layer: u8,
    walls: BinaryHeap<Wall<P>>,
}

impl<P> WallModel<P>
where
    P: PointLike,
{
    pub fn new(layer: u8) -> Self {
        WallModel {
            walls: Default::default(),
            layer,
        }
    }

    pub fn push(&mut self, wall: Wall<P>) {
        self.walls.push(wall);
    }

    pub fn decompose(&self) -> PolyRect {
        for wall in self.walls.iter() {
            if wall.is_forward() {}
        }
        PolyRect {
            rects: vec![],
            layer: self.layer,
        }
    }
}

pub struct WallModeler<P>
where
    P: PointLike,
{
    default_wall_capacity: usize,
    current_wall: Wall<P>,
    wall_model: WallModel<P>,
}

impl<P> WallModeler<P>
where
    P: PointLike,
{
    pub fn build(polygon: &Polygon) -> WallModel<P> {
        let corners = polygon.corners().corners;
        let mut walls_builder = Self::new(polygon.layer, corners.len() / 2);

        for corner in corners.into_iter() {
            walls_builder.push(RectCorner::from_other(&corner));
        }

        walls_builder.finish()
    }

    pub fn new(layer: u8, default_wall_capacity: usize) -> Self {
        WallModeler {
            default_wall_capacity,
            current_wall: Wall::new_forward(0, default_wall_capacity),
            wall_model: WallModel::new(layer),
        }
    }

    fn force_push_forward(&mut self, rect_corner: RectCorner<P>) {
        if self.current_wall.is_reverse() {
            self.wall_model.push(self.current_wall.clone());
            self.current_wall =
                Wall::new_forward(self.wall_model.walls.len(), self.default_wall_capacity);
        }
        self.current_wall.push(rect_corner);
    }

    fn force_push_reverse(&mut self, rect_corner: RectCorner<P>) {
        if self.current_wall.is_forward() {
            self.wall_model.push(self.current_wall.clone());
            self.current_wall =
                Wall::new_reverse(self.wall_model.walls.len(), self.default_wall_capacity);
        }
        self.current_wall.push(rect_corner);
    }

    pub fn push_into_current(&mut self, rect_corner: RectCorner<P>) {
        self.current_wall.push(rect_corner);
    }

    pub fn push(&mut self, rect_corner: RectCorner<P>) {
        match rect_corner.corner_type().outgoing_part() {
            RectDirection::Down => self.force_push_reverse(rect_corner),
            RectDirection::Left => self.push_into_current(rect_corner),
            RectDirection::Right => self.push_into_current(rect_corner),
            RectDirection::Up => self.force_push_forward(rect_corner),
        };
    }

    pub fn finish(mut self) -> WallModel<P> {
        self.wall_model.push(self.current_wall.clone());

        self.wall_model
    }
}
