use crate::decomp::RectCorner;
use crate::shapes::{PointLike, RectDirection};

#[derive(Clone, Copy, Debug)]
pub enum WallAttitude {
    Forward,
    Reverse,
}

#[derive(Clone, Debug)]
pub struct Wall<P>
where
    P: PointLike,
{
    attitude: WallAttitude,
    rect_corners: Vec<RectCorner<P>>,
    /// Index of the walls which match this wall
    ///
    /// If this wall's `attitude` is [`Forward`](WallAttitude), it matches with those
    /// [`Reverse`](WallAttitude) walls which it shadows.
    ///
    /// If this wall's `attitude` is [`Reverse`](WallAttitude), it matches with those
    /// [`Forward`](WallAttitude) walls which shadow it.
    matches: Vec<usize>,
}

impl<P> Wall<P>
where
    P: PointLike,
{
    pub fn new_forward(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Forward,
            rect_corners: Vec::with_capacity(capacity),
            matches: vec![],
        }
    }

    pub fn new_reverse(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Reverse,
            rect_corners: Vec::with_capacity(capacity),
            matches: vec![],
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

    pub fn project<Q: PointLike>(&self, point: &Q) -> Option<(P, RectCorner<P>)> {
        let bottom_most = self.bottom_most();
        let top_most = self.top_most();

        if point.y() <= bottom_most.point().y() && point.y() <= top_most.point().y() {
            for rect_corner in self.rect_corners.iter() {
                let p0 = rect_corner.point();
                let p1 = rect_corner.outgoing();

                if let Some(projected) = point.project_vertical(&p0, &p1) {
                    return Some((P::from_other(&projected), *rect_corner));
                }
            }
        }

        None
    }

    pub fn attitude(&self) -> WallAttitude {
        self.attitude
    }
}
