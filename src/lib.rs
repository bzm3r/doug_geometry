mod bbox;
pub mod shapes;

use crate::shapes::{Point, PointLike, PolyRect, Polygon, Rect, RectDirection};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PolyDecomposer {
    pub points: Vec<Point>,
    pub directions: Vec<RectDirection>,
    pub horizontal_inversions: usize,
    pub vertical_inversions: usize,
}

/// The type of corner in a rectilinear polygon
///
/// It consists of an incoming direction, and an outgoing direction
#[derive(Clone, Copy, Debug)]
pub enum CornerType {
    // rectangle corners (90 degree inner angles)
    DownRight,
    RightUp,
    UpLeft,
    LeftDown,
    // general rectilinear polygon corners (> 90 degree inner angles)
    DownLeft,
    RightDown,
    UpRight,
    LeftUp,
}

impl CornerType {
    pub fn new(incoming: RectDirection, outgoing: RectDirection) -> Self {
        match (incoming, outgoing) {
            (RectDirection::Down, RectDirection::Right) => Self::DownRight,
            (RectDirection::Right, RectDirection::Up) => Self::RightUp,
            (RectDirection::Up, RectDirection::Left) => Self::UpLeft,
            (RectDirection::Left, RectDirection::Down) => Self::LeftDown,
            (RectDirection::Down, RectDirection::Left) => Self::DownLeft,
            (RectDirection::Right, RectDirection::Down) => Self::RightDown,
            (RectDirection::Up, RectDirection::Right) => Self::UpRight,
            (RectDirection::Left, RectDirection::Up) => Self::LeftUp,
            (incoming, outgoing) => panic!("Invalid incoming/outgoing directions for a rectilinear corner! Incoming: {:?}  Outgoing: {:?}", incoming, outgoing),
        }
    }

    pub fn horizontal_part(&self) -> RectDirection {
        match self {
            CornerType::UpLeft
            | CornerType::LeftDown
            | CornerType::DownLeft
            | CornerType::LeftUp => RectDirection::Left,
            _ => RectDirection::Right,
        }
    }

    pub fn vertical_part(&self) -> RectDirection {
        match self {
            CornerType::DownRight
            | CornerType::LeftDown
            | CornerType::DownLeft
            | CornerType::RightDown => RectDirection::Down,
            _ => RectDirection::Up,
        }
    }

    pub fn right_angled(&self) -> bool {
        match self {
            CornerType::DownRight
            | CornerType::RightUp
            | CornerType::UpLeft
            | CornerType::LeftDown => true,
            _ => false,
        }
    }

    pub fn incoming_part(&self) -> RectDirection {
        match self {
            CornerType::DownRight | CornerType::DownLeft => RectDirection::Down,
            CornerType::RightUp | CornerType::RightDown => RectDirection::Right,
            CornerType::UpLeft | CornerType::UpRight => RectDirection::Up,
            CornerType::LeftDown | CornerType::LeftUp => RectDirection::Left,
        }
    }

    pub fn outgoing_part(&self) -> RectDirection {
        match self {
            CornerType::RightDown | CornerType::LeftDown => RectDirection::Down,
            CornerType::UpRight | CornerType::DownRight => RectDirection::Right,
            CornerType::RightUp | CornerType::LeftUp => RectDirection::Up,
            CornerType::UpLeft | CornerType::DownLeft => RectDirection::Left,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RectCorner<P: PointLike> {
    incoming: P,
    outgoing: P,
    center: P,
    corner_type: CornerType,
}

impl<P> RectCorner<P>
where
    P: PointLike,
{
    pub fn new(incoming: P, outgoing: P, center: P) -> Self {
        Self {
            incoming,
            outgoing,
            center,
            corner_type: CornerType::new(
                incoming.simple_directions_to(&center),
                center.simple_directions_to(&outgoing),
            ),
        }
    }

    pub fn from_other<Q: PointLike>(other: &RectCorner<Q>) -> Self {
        Self {
            incoming: P::from_other(&other.incoming),
            outgoing: P::from_other(&other.outgoing),
            center: P::from_other(&other.center),
            corner_type: other.corner_type,
        }
    }
}

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
    fn new_forward(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Forward,
            rect_corners: Vec::with_capacity(capacity),
            matches: vec![],
        }
    }

    fn new_reverse(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Reverse,
            rect_corners: Vec::with_capacity(capacity),
            matches: vec![],
        }
    }

    fn len(&self) -> usize {
        self.rect_corners.len()
    }

    fn push(&mut self, rect_corner: RectCorner<P>) {
        match self.attitude {
            WallAttitude::Forward => {
                if rect_corner.corner_type.vertical_part() == RectDirection::Up {
                    self.rect_corners.push(rect_corner);
                } else {
                    panic!(
                        "Cannot push a {:?} into a Forward vertical wall",
                        rect_corner.corner_type
                    );
                }
            }
            WallAttitude::Reverse => {
                if rect_corner.corner_type.vertical_part() == RectDirection::Down {
                    self.rect_corners.push(rect_corner);
                } else {
                    panic!(
                        "Cannot push a {:?} into a Reverse vertical wall",
                        rect_corner.corner_type
                    );
                }
            }
        }
    }

    fn clear(&mut self) {
        self.rect_corners.clear();
    }

    fn is_forward(&self) -> bool {
        match self.attitude {
            WallAttitude::Forward => true,
            _ => false,
        }
    }

    fn is_reverse(&self) -> bool {
        !self.is_forward()
    }

    fn first(&self) -> &RectCorner<P> {
        &self.rect_corners[0]
    }

    fn last(&self) -> &RectCorner<P> {
        &self.rect_corners[self.rect_corners.len() - 1]
    }

    fn shadows<Q: PointLike>(&self, other: &Wall<Q>) -> bool {
        self.is_forward()
            && !other.is_forward()
            && self.first().center.y() <= other.first().center.y()
            && other.first().center.y() <= self.last().center.y()
    }

    fn project<Q: PointLike>(&self, point: &Q) -> Option<P> {
        let (bottommost, topmost) = match self.attitude {
            WallAttitude::Forward => (self.first(), self.last()),
            WallAttitude::Reverse => (self.last(), self.first()),
        };

        if point.y() <= bottommost.center.y() && point.y() <= topmost.center.y() {
            for wall_point_pair in self.rect_corners.as_slice().windows(2) {
                let [p0, p1]: [_; 2] = wall_point_pair.try_into().ok().unwrap();

                if let Some(projected) = point.project_vertical(&p0.center, &p1.center) {
                    return Some(P::from_other(&projected));
                }
            }
        }

        None
    }
}

pub struct Walls<P>
where
    P: PointLike,
{
    layer: u8,
    walls: Vec<Wall<P>>,
}

impl<P> Walls<P>
where
    P: PointLike,
{
    pub fn new(layer: u8) -> Self {
        Walls {
            walls: Vec::new(),
            layer,
        }
    }

    pub fn push(&mut self, mut wall: Wall<P>) {
        match wall.attitude {
            WallAttitude::Reverse => {
                let next_ix = self.walls.len();
                for (previous_ix, previous_wall) in self.walls.iter_mut().enumerate().rev() {
                    if previous_wall.is_forward() && previous_wall.shadows(&wall) {
                        previous_wall.matches.push(next_ix);
                        wall.matches.push(previous_ix);
                    }
                }
            }
            _ => {}
        }

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

pub struct WallsBuilder<P>
where
    P: PointLike,
{
    default_wall_capacity: usize,
    current_wall: Wall<P>,
    walls: Walls<P>,
}

impl<P> WallsBuilder<P>
where
    P: PointLike,
{
    pub fn build(polygon: &Polygon) -> Walls<P> {
        let corners = polygon.corners();
        let mut walls_builder = Self::new(polygon.layer, corners.len() / 2);

        for corner in corners.into_iter() {
            walls_builder.push(RectCorner::from_other(&corner));
        }

        walls_builder.finish()
    }

    pub fn new(layer: u8, default_wall_capacity: usize) -> Self {
        WallsBuilder {
            default_wall_capacity,
            current_wall: Wall::new_forward(default_wall_capacity),
            walls: Walls::new(layer),
        }
    }

    fn force_push_forward(&mut self, rect_corner: RectCorner<P>) {
        if self.current_wall.is_reverse() {
            self.walls.push(self.current_wall.clone());
            self.current_wall = Wall::new_forward(self.default_wall_capacity);
        }
        self.current_wall.push(rect_corner);
    }

    fn force_push_reverse(&mut self, rect_corner: RectCorner<P>) {
        if self.current_wall.is_forward() {
            self.walls.push(self.current_wall.clone());
            self.current_wall = Wall::new_reverse(self.default_wall_capacity);
        }
        self.current_wall.push(rect_corner);
    }

    pub fn push_into_current(&mut self, rect_corner: RectCorner<P>) {
        self.current_wall.push(rect_corner);
    }

    pub fn push(&mut self, rect_corner: RectCorner<P>) {
        match rect_corner.corner_type.outgoing_part() {
            RectDirection::Down => self.force_push_reverse(rect_corner),
            RectDirection::Left => self.push_into_current(rect_corner),
            RectDirection::Right => self.push_into_current(rect_corner),
            RectDirection::Up => self.force_push_forward(rect_corner),
        };
    }

    pub fn finish(mut self) -> Walls<P> {
        self.walls.push(self.current_wall.clone());

        self.walls
    }
}
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
