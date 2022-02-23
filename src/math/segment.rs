use std::fmt;

use super::Point;

/// A line segment, defined by its two end points
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Segment<const D: usize> {
    a: Point<D>,
    b: Point<D>,
}

impl Segment<2> {
    /// Convert the 2-dimensional segment to a Parry segment
    pub fn to_parry(&self) -> parry2d_f64::shape::Segment {
        [self.a.to_na(), self.b.to_na()].into()
    }

    pub fn points(&self) -> [Point<2>; 2] {
        [self.a, self.b]
    }
}

impl Segment<3> {
    /// Convert the 3-dimensional segment to a Parry segment
    pub fn to_parry(&self) -> parry3d_f64::shape::Segment {
        [self.a.to_na(), self.b.to_na()].into()
    }

    pub fn points(&self) -> [Point<3>; 2] {
        [self.a, self.b]
    }
}

impl From<[Point<2>; 2]> for Segment<2> {
    fn from(points: [Point<2>; 2]) -> Self {
        Self {
            a: points[0],
            b: points[1],
        }
    }
}

impl From<[Point<3>; 2]> for Segment<3> {
    fn from(points: [Point<3>; 2]) -> Self {
        Self {
            a: points[0],
            b: points[1],
        }
    }
}

impl<const D: usize> fmt::Debug for Segment<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?} -> {:?}]", self.a, self.b)
    }
}