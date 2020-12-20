use itertools::iproduct;
use crate::relative::Relative;
use std::ops::Add;

pub type HyperCellTuple = (isize, isize, isize, isize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HyperCell(isize, isize, isize, isize);

impl From<(isize, isize)> for HyperCell {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y, 0, 0)
    }
}

impl From<HyperCellTuple> for HyperCell {
    fn from((x, y, z, w): HyperCellTuple) -> Self {
        Self(x, y, z, w)
    }
}

impl Add for HyperCell {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        HyperCell(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl Add<HyperCellTuple> for HyperCell {
    type Output = Self;

    fn add(self, rhs: HyperCellTuple) -> Self::Output {
        HyperCell(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl Relative for HyperCell {
    fn adjacent(&self) -> Vec<HyperCell> {
        iproduct!(-1isize..=1, -1isize..=1, -1isize..=1, -1isize..=1)
            .filter(|ct| *ct != (0, 0, 0, 0))
            .map(|ct| *self + ct)
            .collect()
    }
}
