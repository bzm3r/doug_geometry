pub mod poly_index;
use poly_index::Vix;
use std::iter::once;

/// There are two types`X` of vertices in a closed polygon: `X` and `C`. `X`
/// represents a  vertex that is conveX, while `C` represents a vertex that is
/// ConCave.
///
/// Because the existence of a `C` changes the shape from being convex to
/// non-convex non-convex, because the edge structure at an `C` point is
/// non-convex. In order for an `n > 2` shape to close, insertion of an `C`
/// requires insertion of a `X` to reverse the effect of the `C`. In this sense,
/// `C` and `X` are "annihilating opposites", and the total number of `X` points
/// after all possible `XC` annihilations is invariant: `n` (the number of
/// points we began with in our convex shape).
#[derive(Clone, Copy, Debug)]
pub enum Vertex {
    X,
    C,
}

/// Both [`TwistAction::XC`] and [`TwistAction::CX`] indistinguishable (or
/// equivalent) for an [`INIT`] (purely convex) configuration (i.e. no
/// [`LiquidPoint::C`]s.). However, once we perform a `TwistAction` on a
/// [`LiquidPoly`], the action of `XC` and `CX` can be distinguished, and
/// therefore we need both available to us.
pub enum Twist {
    /// Inserts a `(X, C)` pair [`LiquidPoly`]
    XC,
    /// Inserts a `(C, X)` into a [`LiquidPoly`]
    CX,
}

impl IntoIterator for Twist {
    type Item = Vertex;

    type IntoIter = <[Vertex; 2] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        use Vertex::*;
        match self {
            Twist::XC => [X, C].into_iter(),
            Twist::CX => [C, X].into_iter(),
        }
    }
}

pub const INIT: [Vertex; 4] = [Vertex::X; 4];

/// A "liquid polygon" is an abstract representation of a polygon, where only
/// topological information regarding edge concavity is retained. Therefore, a
/// sequence of n-`X` points can represent a variety of convex, closed polygons.
/// For example:
/// * `n = 0`: nothing
/// * `n = 1`: an arbitrary point in space
/// * `n = 2`: a line segment
/// * `n = 3`: a triangle
/// * `n = 4`: a rectangle
///
/// Out of these we are most interested in the `n = 4` case. However, sometimes
/// it will also be necessary to consider the `n = 2` case when we are dealing
/// with a particular wall of a rectangle.
pub struct LiquidPoly {
    /// The sequences of vertices representing this polygon.
    vertices: Vec<Vertex>,
}

impl LiquidPoly {
    /// Create a new polygon.
    pub fn init() -> LiquidPoly {
        LiquidPoly {
            vertices: Vec::from(INIT),
        }
    }

    /// The number of vertices in this poly.
    pub fn len(&self) -> Vix {
        self.vertices.len().into()
    }

    /// A twist is applied to a particular edge of the liquid polygon, Each
    /// edge has a 1-to-1 relationship with its source vertex, so we can
    /// consider vertices and edges to be equivalent in this sense. Therefore,
    /// the index supplied to this action can be thought of as applying to an
    /// edge, or applying to a vertex.
    pub fn twist(mut self, action: Twist, ix: Vix) -> Self {
        self.vertices = self
            .vertices
            .splice(ix.0..ix.0, once(self[ix]).chain(action))
            .collect();
        self
    }
}
