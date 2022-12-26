use crate::shapes::{PointLike, RectDirection};

/// The type of corner in a rectilinear polygon
///
/// It consists of an incoming direction, and an outgoing direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
            (incoming, outgoing) => panic!("Invalid incoming/outgoing directions for a rectilinear corner! Incoming: {incoming:?}  Outgoing: {outgoing:?}"),
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RectCorner<P: PointLike> {
    incoming: P,
    outgoing: P,
    point: P,
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
            point: center,
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
            point: P::from_other(&other.point),
            corner_type: other.corner_type,
        }
    }

    pub fn corner_type(&self) -> CornerType {
        self.corner_type
    }

    pub fn point(&self) -> P {
        self.point
    }

    pub fn incoming(&self) -> P {
        self.incoming
    }

    pub fn outgoing(&self) -> P {
        self.outgoing
    }

    pub fn project_onto_vertical_outgoing<Q: PointLike>(&self, p: Q) -> Option<P> {
        let (top, bottom) = match self.corner_type.outgoing_part() {
            RectDirection::Down => (self.point.y(), self.outgoing.y()),
            RectDirection::Up => (self.outgoing.y(), self.point.y()),
            RectDirection::Left | RectDirection::Right => {
                return None;
            }
        };

        (bottom <= p.y() && p.y() <= top).then_some(P::new(self.point.x(), p.y()))
    }
}

pub struct Corners<P: PointLike> {
    vertical_inversions: usize,
    horizontal_inversions: usize,
    min_x_max_y_index: usize,
    min_y_min_x_index: usize,
    pub corners: Vec<RectCorner<P>>,
}

impl<P: PointLike> Corners<P> {
    pub fn min_x_max_y_index(corners: &[RectCorner<P>]) -> usize {
        let min_x = corners
            .iter()
            .map(|corner| corner.point().x())
            .min()
            .unwrap();

        corners
            .iter()
            .enumerate()
            .filter_map(|(ix, corner)| (corner.point().x() == min_x).then_some((ix, corner)))
            .max_by_key(|(_ix, corner)| corner.point().y())
            .map(|(ix, _corner)| ix)
            .unwrap()
    }

    pub fn min_y_min_x_index(corners: &[RectCorner<P>]) -> usize {
        let min_y = corners
            .iter()
            .map(|corner| corner.point().y())
            .min()
            .unwrap();

        corners
            .iter()
            .enumerate()
            .filter_map(|(ix, corner)| (corner.point().y() == min_y).then_some((ix, corner)))
            .min_by_key(|(_ix, corner)| corner.point().x())
            .map(|(ix, _corner)| ix)
            .unwrap()
    }

    pub fn count_horizontal_inversions(
        left_most_bottom_most_ix: usize,
        corners: &[RectCorner<P>],
    ) -> usize {
        let mut horizontal_inversions = 0;
        let mut last_horizontal = corners[left_most_bottom_most_ix]
            .corner_type()
            .horizontal_part();
        let n_corners = corners.len();
        for i in 1..n_corners {
            let this_horizontal = corners[(left_most_bottom_most_ix + i) % n_corners]
                .corner_type()
                .horizontal_part();

            if this_horizontal != last_horizontal {
                last_horizontal = this_horizontal;
                horizontal_inversions += 1;
            }
        }
        horizontal_inversions
    }

    pub fn count_vertical_inversions(
        bottom_most_right_most_ix: usize,
        corners: &[RectCorner<P>],
    ) -> usize {
        let mut vertical_inversions = 0;
        let mut last_vertical = corners[bottom_most_right_most_ix]
            .corner_type()
            .vertical_part();
        let n_corners = corners.len();
        for i in 1..n_corners {
            let this_vertical = corners[(bottom_most_right_most_ix + i) % n_corners]
                .corner_type()
                .vertical_part();

            if this_vertical != last_vertical {
                last_vertical = this_vertical;
                vertical_inversions += 1;
            }
        }
        vertical_inversions
    }

    pub fn new(corners: Vec<RectCorner<P>>) -> Self {
        let min_x_max_y_index = Self::min_x_max_y_index(&corners);
        let min_y_min_x_index = Self::min_y_min_x_index(&corners);
        let horizontal_inversions = Self::count_horizontal_inversions(min_x_max_y_index, &corners);
        let vertical_inversions = Self::count_vertical_inversions(min_y_min_x_index, &corners);

        Self {
            vertical_inversions,
            horizontal_inversions,
            min_x_max_y_index,
            min_y_min_x_index,
            corners,
        }
    }
}
