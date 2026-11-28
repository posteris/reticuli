use crate::common::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    #[inline]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn width(self) -> u16 {
        self.width
    }

    #[inline]
    pub const fn height(self) -> u16 {
        self.height
    }

    #[inline]
    pub const fn area(self) -> u32 {
        self.width as u32 * self.height as u32
    }

    #[inline]
    pub const fn contains(&self, position: Position) -> bool {
        position.x() < self.width && position.y() < self.height
    }

    #[inline]
    pub const fn last(self) -> Position {
        Position::new(self.width - 1, self.height - 1)
    }
}

impl From<Size> for (u16, u16) {
    fn from(size: Size) -> Self {
        (size.width(), size.height())
    }
}
