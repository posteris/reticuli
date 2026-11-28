use std::ops::Add;

use crate::common::size::Size;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub const ZERO: Self = Self::new(0, 0);

    #[inline]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn x(&self) -> u16 {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> u16 {
        self.y
    }

    #[inline]
    pub const fn flatten(self, size: Size) -> Option<u32> {
        if size.contains(self) {
            Some(self.y as u32 * size.width() as u32 + self.x as u32)
        } else {
            None
        }
    }

    #[inline]
    pub const fn with_x(self, x: u16) -> Self {
        Self { x, y: self.y }
    }

    #[inline]
    pub const fn with_y(self, y: u16) -> Self {
        Self { x: self.x, y }
    }

    #[inline]
    pub const fn add_x(self, x: u16) -> Self {
        Self {
            x: x + self.x,
            y: self.y,
        }
    }

    #[inline]
    pub const fn add_y(self, y: u16) -> Self {
        Self {
            x: self.x,
            y: y + self.y,
        }
    }

    #[inline]
    pub const fn distance(self, to: Self) -> Size {
        Size::new(self.x.abs_diff(to.x), self.y.abs_diff(to.y))
    }
}

impl Add for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<(u16, u16)> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: (u16, u16)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}
