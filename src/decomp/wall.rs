use crate::decomp::RectCorner;
use crate::shapes::{PointLike, RectDirection};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallAttitude {
    Forward,
    Reverse,
}

#[derive(Clone, Debug)]
pub struct Wall<P>
where
    P: PointLike,
{
    /// The index of the wall
    generation_index: usize,
    attitude: WallAttitude,
    rect_corners: Vec<RectCorner<P>>,
}

pub struct ProjectionResult<P: PointLike> {
    /// Index of the corner (in a wall) whose outgoing segment contains the projected point
    pub corner_ix: usize,
    /// The projected point
    pub projected: P,
}

impl<P: PointLike> ProjectionResult<P> {
    pub fn new(corner_ix: usize, projected: P) -> Self {
        Self {
            corner_ix,
            projected,
        }
    }
}

impl<P> Wall<P>
where
    P: PointLike,
{
    pub fn new_forward(generation_index: usize, capacity: usize) -> Self {
        Wall {
            generation_index,
            attitude: WallAttitude::Forward,
            rect_corners: Vec::with_capacity(capacity),
        }
    }

    pub fn new_reverse(generation_index: usize, capacity: usize) -> Self {
        Wall {
            generation_index,
            attitude: WallAttitude::Reverse,
            rect_corners: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.rect_corners.len()
    }

    pub fn push(&mut self, rect_corner: RectCorner<P>) {
        let corner_type = rect_corner.corner_type();
        match self.attitude {
            WallAttitude::Forward => {
                if corner_type.vertical_part() == RectDirection::Up {
                    self.rect_corners.push(rect_corner);
                } else {
                    panic!(
                        "Cannot push a {:?} into a Forward vertical wall",
                        corner_type
                    );
                }
            }
            WallAttitude::Reverse => {
                if corner_type.vertical_part() == RectDirection::Down {
                    self.rect_corners.push(rect_corner);
                } else {
                    panic!(
                        "Cannot push a {:?} into a Reverse vertical wall",
                        corner_type
                    );
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.rect_corners.clear();
    }

    pub fn is_forward(&self) -> bool {
        match self.attitude {
            WallAttitude::Forward => true,
            _ => false,
        }
    }

    pub fn is_reverse(&self) -> bool {
        !self.is_forward()
    }

    pub fn first(&self) -> &RectCorner<P> {
        &self.rect_corners[0]
    }

    pub fn last(&self) -> &RectCorner<P> {
        &self.rect_corners[self.rect_corners.len() - 1]
    }

    pub fn bottom_most(&self) -> &RectCorner<P> {
        match self.attitude {
            WallAttitude::Forward => self.first(),
            WallAttitude::Reverse => self.last(),
        }
    }

    pub fn top_most(&self) -> &RectCorner<P> {
        match self.attitude {
            WallAttitude::Forward => self.last(),
            WallAttitude::Reverse => self.first(),
        }
    }

    pub fn shadows<Q: PointLike>(&self, other: &Wall<Q>) -> bool {
        self.is_forward()
            && !other.is_forward()
            && self.first().point().y() <= other.first().point().y()
            && other.first().point().y() <= self.last().point().y()
    }

    pub fn project_onto_wall<Q: PointLike>(&self, point: Q) -> Option<ProjectionResult<P>> {
        if self.interacts_with(point) {
            for (ix, rect_corner) in self.rect_corners.iter().enumerate() {
                if let Some(projected) = rect_corner.project_onto_vertical_outgoing(point) {
                    return Some(ProjectionResult::new(ix, projected));
                }
            }
        }

        None
    }

    pub fn attitude(&self) -> WallAttitude {
        self.attitude
    }

    pub fn interacts_with<Q: PointLike>(&self, point: Q) -> bool {
        self.bottom_most().point().y() <= point.y() && point.y() <= self.top_most().point().y()
    }

    pub fn left_of_wall<Q: PointLike>(&self, other: &Wall<Q>) -> Option<bool> {
        if let Some(projection_result) = self.project_onto_wall(other.top_most().point()) {
            Some(projection_result.projected.x() < other.top_most().point().x())
        } else if let Some(projection_result) = self.project_onto_wall(other.bottom_most().point())
        {
            Some(projection_result.projected.x() < other.bottom_most().point().x())
        } else {
            None
        }
    }
}

impl<P: PointLike> PartialEq for Wall<P> {
    fn eq(&self, other: &Self) -> bool {
        self.generation_index == other.generation_index
    }
}

impl<P: PointLike> PartialOrd for Wall<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(projection_result) = self.project_onto_wall(other.top_most().point()) {
            match projection_result
                .projected
                .x()
                .cmp(&other.top_most().point().x())
            {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Equal => None,
                Ordering::Greater => Some(Ordering::Greater),
            }
        } else if let Some(projection_result) = self.project_onto_wall(other.bottom_most().point())
        {
            match projection_result
                .projected
                .x()
                .cmp(&other.bottom_most().point().x())
            {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Equal => None,
                Ordering::Greater => Some(Ordering::Greater),
            }
        } else {
            None
        }
    }
}

impl<P: PointLike> Eq for Wall<P> {}

impl<P: PointLike> Ord for Wall<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        } else {
            self.generation_index.cmp(&other.generation_index)
        }
    }
}
