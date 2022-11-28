mod bbox;
pub mod shapes;

use crate::shapes::{Point, PointLike, Poly, RectDirection};

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
pub enum Corner {
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

impl Corner {
    pub fn horizontal_part(&self) -> RectDirection {
        match self {
            Corner::UpLeft | Corner::LeftDown | Corner::DownLeft | Corner::LeftUp => {
                RectDirection::Left
            }
            _ => RectDirection::Right,
        }
    }

    pub fn vertical_part(&self) -> RectDirection {
        match self {
            Corner::DownRight | Corner::LeftDown | Corner::DownLeft | Corner::RightDown => {
                RectDirection::Down
            }
            _ => RectDirection::Up,
        }
    }

    pub fn right_angled(&self) -> bool {
        match self {
            Corner::DownRight | Corner::RightUp | Corner::UpLeft | Corner::LeftDown => true,
            _ => false,
        }
    }

    pub fn incoming_part(&self) -> RectDirection {
        match self {
            Corner::DownRight | Corner::DownLeft => RectDirection::Down,
            Corner::RightUp | Corner::RightDown => RectDirection::Right,
            Corner::UpLeft | Corner::UpRight => RectDirection::Up,
            Corner::LeftDown | Corner::LeftUp => RectDirection::Left,
        }
    }

    pub fn outgoing_part(&self) -> RectDirection {
        match self {
            Corner::RightDown | Corner::LeftDown => RectDirection::Down,
            Corner::UpRight | Corner::DownRight => RectDirection::Right,
            Corner::RightUp | Corner::LeftUp => RectDirection::Up,
            Corner::UpLeft | Corner::DownLeft => RectDirection::Left,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WallAttitude {
    Forward,
    Reverse,
}

#[derive(Clone, Copy, Debug)]
pub enum WallOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub struct Wall {
    attitude: WallAttitude,
    orientation: WallOrientation,
    points: Vec<Point>,
    corners: Vec<Corner>,
    /// Index of the walls which match this wall
    ///
    /// If this wall's `attitude` is [`Forward`](WallAttitude), it matches with those
    /// [`Reverse`](WallAttitude) walls (of the same `orientation`) which it shadows.
    ///
    /// If this wall's `attitude` is [`Reverse`](WallAttitude), it matches with those
    /// [`Forward`](WallAttitude) walls (of the same `orientation`) which shadow it.
    matches: Vec<usize>,
}

impl Wall {
    fn new_horizontal_forward(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Forward,
            orientation: WallOrientation::Horizontal,
            points: Vec::with_capacity(capacity),
            corners: Vec::with_capacity(capacity),
            matches: vec![],
        }
    }

    fn new_horizontal_reverse(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Reverse,
            orientation: WallOrientation::Horizontal,
            points: Vec::with_capacity(capacity),
            corners: Vec::with_capacity(capacity),
            matches: vec![],
        }
    }

    fn new_vertical_forward(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Forward,
            orientation: WallOrientation::Vertical,
            points: Vec::with_capacity(capacity),
            corners: Vec::with_capacity(capacity),
            matches: vec![],
        }
    }

    fn new_vertical_reverse(capacity: usize) -> Self {
        Wall {
            attitude: WallAttitude::Reverse,
            orientation: WallOrientation::Vertical,
            points: Vec::with_capacity(capacity),
            corners: Vec::with_capacity(capacity),
            matches: vec![],
        }
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn push(&mut self, point: Point, corner: Corner) {
        match (self.orientation, self.attitude) {
            (WallOrientation::Horizontal, WallAttitude::Forward) => {
                if corner.horizontal_part() == RectDirection::Right {
                    self.points.push(point);
                    self.corners.push(corner);
                } else {
                    panic!("Cannot push a {:?} into a Forward horizontal wall", corner);
                }
            }
            (WallOrientation::Horizontal, WallAttitude::Reverse) => {
                if corner.horizontal_part() == RectDirection::Left {
                    self.points.push(point);
                    self.corners.push(corner);
                } else {
                    panic!("Cannot push a {:?} into a Reverse horizontal wall", corner);
                }
            }
            (WallOrientation::Vertical, WallAttitude::Forward) => {
                if corner.vertical_part() == RectDirection::Up {
                    self.points.push(point);
                    self.corners.push(corner);
                } else {
                    panic!("Cannot push a {:?} into a Forward vertical wall", corner);
                }
            }
            (WallOrientation::Vertical, WallAttitude::Reverse) => {
                if corner.vertical_part() == RectDirection::Down {
                    self.points.push(point);
                    self.corners.push(corner);
                } else {
                    panic!("Cannot push a {:?} into a Reverse vertical wall", corner);
                }
            }
        }
    }

    fn clear(&mut self) {
        self.points.clear();
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

    fn is_horizontal(&self) -> bool {
        match self.orientation {
            WallOrientation::Horizontal => true,
            _ => false,
        }
    }

    fn first(&self) -> &Point {
        &self.points[0]
    }

    fn last(&self) -> &Point {
        &self.points[self.points.len() - 1]
    }

    fn shadows(&self, other: &Wall) -> bool {
        self.is_forward()
            && match self.orientation {
                WallOrientation::Horizontal => {
                    // TODO: confirm that other.first().x <= self.last().x and not other.first().x < self.last().x
                    !other.is_forward()
                        && self.first().x <= other.first().x
                        && other.first().x <= self.last().x
                }
                WallOrientation::Vertical => {
                    // TODO: confirm that other.first().y <= self.last().y and not other.first().y < self.last().y
                    !other.is_forward()
                        && self.first().y <= other.first().y
                        && other.first().y <= self.last().y
                }
            }
    }

    fn project(&self, point: &Point) -> Option<Point> {
        match self.orientation {
            WallOrientation::Horizontal => {
                let (leftmost, rightmost) = match self.attitude {
                    WallAttitude::Forward => (self.first(), self.last()),
                    WallAttitude::Reverse => (self.last(), self.first()),
                };

                if point.x() <= leftmost.x() && point.x() <= rightmost.x() {
                    for wall_point_pair in self.points.as_slice().windows(2) {
                        let [p0, p1]: [_; 2] = wall_point_pair.try_into().ok().unwrap();

                        if let Some(projected) = point.project_horizontal(&p0, &p1) {
                            return Some(projected);
                        }
                    }
                }
            }
            WallOrientation::Vertical => {
                let (bottommost, topmost) = match self.attitude {
                    WallAttitude::Forward => (self.first(), self.last()),
                    WallAttitude::Reverse => (self.last(), self.first()),
                };

                if point.y() <= bottommost.y() && point.y() <= topmost.y() {
                    for wall_point_pair in self.points.as_slice().windows(2) {
                        let [p0, p1]: [_; 2] = wall_point_pair.try_into().ok().unwrap();

                        if let Some(projected) = point.project_vertical(&p0, &p1) {
                            return Some(projected);
                        }
                    }
                }
            }
        }

        None
    }
}

pub struct Walls {
    horizontal_walls: Vec<Wall>,
    vertical_walls: Vec<Wall>,
}

impl Walls {
    pub fn new() -> Self {
        Walls {
            horizontal_walls: Vec::new(),
            vertical_walls: Vec::new(),
        }
    }

    pub fn push(&mut self, mut wall: Wall) {
        match wall.orientation {
            WallOrientation::Horizontal => {
                match wall.attitude {
                    WallAttitude::Reverse => {
                        let next_ix = self.horizontal_walls.len();
                        for (previous_ix, previous_wall) in
                            self.horizontal_walls.iter_mut().enumerate().rev()
                        {
                            if previous_wall.is_forward() && previous_wall.shadows(&wall) {
                                previous_wall.matches.push(next_ix);
                                wall.matches.push(previous_ix);
                            }
                        }
                    }
                    _ => {}
                }

                self.horizontal_walls.push(wall);
            }
            WallOrientation::Vertical => {
                match wall.attitude {
                    WallAttitude::Reverse => {
                        let next_ix = self.vertical_walls.len();
                        for (previous_ix, previous_wall) in
                            self.vertical_walls.iter_mut().enumerate().rev()
                        {
                            if previous_wall.is_forward() && previous_wall.shadows(&wall) {
                                previous_wall.matches.push(next_ix);
                                wall.matches.push(previous_ix);
                            }
                        }
                    }
                    _ => {}
                }

                self.vertical_walls.push(wall);
            }
        }
    }
}

pub struct WallsBuilder {
    default_wall_capacity: usize,
    current_horizontal: Wall,
    current_vertical: Wall,
    walls: Walls,
}

impl WallsBuilder {
    pub fn new(default_wall_capacity: usize) -> Self {
        WallsBuilder {
            default_wall_capacity,
            current_horizontal: Wall::new_horizontal_forward(default_wall_capacity),
            current_vertical: Wall::new_vertical_forward(default_wall_capacity),
            walls: Walls::new(),
        }
    }

    fn push_horizontal_forward(&mut self, point: Point, corner: Corner) {
        if self.current_horizontal.is_reverse() {
            self.walls
                .horizontal_walls
                .push(self.current_horizontal.clone());
            self.current_horizontal = Wall::new_horizontal_forward(self.default_wall_capacity);
        }
        self.current_horizontal.push(point, corner);
    }

    fn push_horizontal_reverse(&mut self, point: Point, corner: Corner) {
        if self.current_horizontal.is_forward() {
            self.walls
                .horizontal_walls
                .push(self.current_horizontal.clone());
            self.current_horizontal = Wall::new_horizontal_reverse(self.default_wall_capacity);
        }
        self.current_horizontal.push(point, corner);
    }

    fn push_vertical_forward(&mut self, point: Point, corner: Corner) {
        if self.current_vertical.is_reverse() {
            self.walls
                .vertical_walls
                .push(self.current_vertical.clone());
            self.current_vertical = Wall::new_vertical_forward(self.default_wall_capacity);
        }
        self.current_vertical.push(point, corner);
    }

    fn push_vertical_reverse(&mut self, point: Point, corner: Corner) {
        if self.current_vertical.is_forward() {
            self.walls
                .vertical_walls
                .push(self.current_vertical.clone());
            self.current_vertical = Wall::new_vertical_reverse(self.default_wall_capacity);
        }
        self.current_vertical.push(point, corner);
    }
0
    pub fn maybe_push_horizontal_forward(&mut self, point: Point, corner: Corner) {
        if self.current_horizontal.is_forward() {
            self.current_horizontal.push(point, corner);
        }
    }

    pub fn maybe_push_horizontal_reverse(&mut self, point: Point, corner: Corner) {
        if self.current_horizontal.is_reverse() {
            self.current_horizontal.push(point, corner);
        }
    }

    pub fn maybe_push_vertical_forward(&mut self, point: Point, corner: Corner) {
        if self.current_vertical.is_forward() {
            self.current_vertical.push(point, corner);
        }
    }

    pub fn maybe_push_vertical_reverse(&mut self, point: Point, corner: Corner) {
        if self.current_vertical.is_reverse() {
            self.current_vertical.push(point, corner);
        }
    }

    pub fn push(&mut self, point: Point, corner: Corner) {
        match corner.outgoing_part() {
            RectDirection::Down => self.push_vertical_reverse(point, corner),
            RectDirection::Left => self.push_horizontal_reverse(point, corner),
            RectDirection::Right => self.push_horizontal_forward(point, corner),
            RectDirection::Up => self.push_vertical_forward(point, corner),
        };

        match corner.incoming_part() {
            RectDirection::Down => self.maybe_push_vertical_reverse(point, corner),
            RectDirection::Left => self.maybe_push_horizontal_reverse(point, corner),
            RectDirection::Right => self.maybe_push_horizontal_forward(point, corner),
            RectDirection::Up => self.maybe_push_vertical_forward(point, corner),
        };
    }

    pub fn finish(self) -> Walls {
        assert_eq!(self.current_horizontal.points.len(), 0);
        assert_eq!(self.current_vertical.points.len(), 0);

        self.walls
    }
}

impl PolyDecomposer {
    pub fn from_poly(poly: Poly) -> PolyDecomposer {
        let mut points = poly.points.clone();
        // TODO: remove this assert because all shape verification should happen prior to archiving
        assert!(points.len() > 3);

        let mut directions = Vec::<RectDirection>::with_capacity(points.len() - 1);
        let mut corners = Vec::<Corner>::with_capacity(points.len());
        corners.push(Corner::DownRight);

        use RectDirection::*;
        let mut last_horizontal_ix: Option<usize> = None;
        let mut last_vertical_ix: Option<usize> = None;

        let mut vertical_walls = Walls::new(WallType::Vertical {
            attitude: RectDirection::Up,
        });
        let mut current_vertical_wall = Wall::with_capacity(points.len() / 2);
        let mut horizontal_walls = Walls::new(WallType::Horizontal {
            attitude: RectDirection::Right,
        });
        let mut current_horizontal_wall = Wall::with_capacity(points.len() / 2);

        for point_window in points.as_slice().windows(2) {
            let [p0, p1]: [_; 2] = point_window.try_into().ok().unwrap();

            let direction = p0.simple_directions_to(&p1);

            if !directions.is_empty() {
                match direction {
                    Left | Right => {
                        if let Some(ix) = last_horizontal_ix {
                            if directions[ix] != direction {
                                horizontal_walls.push(current_horizontal_wall.clone());
                                current_horizontal_wall.clear();
                            }
                        }

                        directions.push(direction);
                        last_horizontal_ix.replace(directions.len() - 1);
                    }
                    Up | Down => {
                        if let Some(ix) = last_vertical_ix {
                            if directions[ix] != direction {
                                vertical_inversions += 1;
                            }
                        }
                        directions.push(direction);
                        last_vertical_ix.replace(directions.len() - 1);
                    }
                }
            } else {
                directions.push(direction)
            }
        }

        PolyDecomposer { points, directions }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct LineSeg {
    p0: Point,
    p1: Point,
}
