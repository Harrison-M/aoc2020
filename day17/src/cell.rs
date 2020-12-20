use itertools::iproduct;
use crate::relative::Relative;
use std::ops::Add;

pub type CellTuple = (isize, isize, isize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Cell(isize, isize, isize);

impl From<(isize, isize)> for Cell {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y, 0)
    }
}

impl From<CellTuple> for Cell {
    fn from((x, y, z): CellTuple) -> Self {
        Self(x, y, z)
    }
}

impl Add for Cell {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cell(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<CellTuple> for Cell {
    type Output = Self;

    fn add(self, rhs: CellTuple) -> Self::Output {
        Cell(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Relative for Cell {
    fn adjacent(&self) -> Vec<Cell> {
        iproduct!(-1isize..=1, -1isize..=1, -1isize..=1)
            .filter(|ct| *ct != (0, 0, 0))
            .map(|ct| *self + ct)
            .collect()
    }
}
