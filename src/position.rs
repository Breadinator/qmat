#![warn(clippy::all, clippy::pedantic)]

use std::fmt::{self, Debug, Display, Formatter};

#[derive(PartialEq)]
pub struct Position(pub usize, pub usize);

impl Position {
    #[must_use]
    pub fn up(&self) -> Self {
        Position(self.0 - 1, self.1)
    }
    #[must_use]
    pub fn up_n(&self, n: usize) -> Self {
        Position(self.0 - n, self.1)
    }

    #[must_use]
    pub fn down(&self) -> Self {
        Position(self.0 + 1, self.1)
    }
    #[must_use]
    pub fn down_n(&self, n: usize) -> Self {
        Position(self.0 + n, self.1)
    }

    #[must_use]
    pub fn left(&self) -> Self {
        Position(self.0, self.1 - 1)
    }
    #[must_use]
    pub fn left_n(&self, n: usize) -> Self {
        Position(self.0, self.1 - n)
    }

    #[must_use]
    pub fn right(&self) -> Self {
        Position(self.0, self.1 + 1)
    }
    #[must_use]
    pub fn right_n(&self, n: usize) -> Self {
        Position(self.0, self.1 + n)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("({}, {})", self.0, self.1))
    }
}
impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("Position {{ {}, {} }}", self.0, self.1))
    }
}
