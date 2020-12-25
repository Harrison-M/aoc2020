use itertools::iproduct;
use num::{range_inclusive, Signed, PrimInt};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point2D<T>(pub T, pub T);

impl<T: Add<T, Output=T>> Add for Point2D<T> {
    type Output = Point2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Add<T, Output=T>> Add<(T, T)> for Point2D<T> {
    type Output = Point2D<T>;

    fn add(self, rhs: (T, T)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Add<T, Output=T> + Copy> AddAssign for Point2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Add<T, Output=T> + Copy> AddAssign<(T, T)> for Point2D<T> {
    fn add_assign(&mut self, rhs: (T, T)) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<T, Output=T>> Sub for Point2D<T> {
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Sub<T, Output=T>> Sub<(T, T)> for Point2D<T> {
    type Output = Point2D<T>;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Sub<T, Output=T> + Copy> SubAssign for Point2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Sub<T, Output=T> + Copy> SubAssign<(T, T)> for Point2D<T> {
    fn sub_assign(&mut self, rhs: (T, T)) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Mul<T, Output=T> + Copy> Mul<T> for Point2D<T> {
    type Output = Point2D<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Mul<T, Output=T> + Copy> MulAssign<T> for Point2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Div<T, Output=T> + Copy> Div<T> for Point2D<T> {
    type Output = Point2D<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: Div<T, Output=T> + Copy> DivAssign<T> for Point2D<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: PrimInt + Signed> Point2D<T> {
    /// Given the point is part of a discrete grid, find the adjacent points.
    /// "Adjacent" here is considered to be the points in both the cardinal and
    /// intercardinal directions.
    pub fn adjacent_points(&self) -> Vec<Self> {
        let end: T = T::one();
        let start: T = -end;
        let zero = T::zero();
        let range = range_inclusive(start, end);
        iproduct!(range.clone(), range)
            .filter(|ct| *ct != (zero, zero))
            .map(|ct| *self + ct)
            .collect()
    }
}
