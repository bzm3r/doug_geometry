use crate::shapes::{PointLike, RectDirection};

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
}
