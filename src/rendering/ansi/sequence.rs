use crate::{
    common::{color::Color, position::Position},
    rendering::ansi::digits::Digits,
};

pub trait Sequence: Sized {
    fn encode(self, buffer: &mut Vec<u8>);
    fn length(self) -> usize;
}

pub struct Foreground(pub Color);
impl Sequence for Foreground {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        let r_digits = Digits::lookup_u8(self.0.r());
        let g_digits = Digits::lookup_u8(self.0.g());
        let b_digits = Digits::lookup_u8(self.0.b());

        buffer.extend_from_slice(b"\x1b[38;2;");
        buffer.extend_from_slice(r_digits.ascii());
        buffer.push(b';');
        buffer.extend_from_slice(g_digits.ascii());
        buffer.push(b';');
        buffer.extend_from_slice(b_digits.ascii());
        buffer.push(b'm');
    }

    #[inline]
    fn length(self) -> usize {
        let r_digits = Digits::lookup_u8(self.0.r());
        let g_digits = Digits::lookup_u8(self.0.g());
        let b_digits = Digits::lookup_u8(self.0.b());

        10 + r_digits.count() + g_digits.count() + b_digits.count()
    }
}

pub struct Background(pub Color);
impl Sequence for Background {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        let r_digits = Digits::lookup_u8(self.0.r());
        let g_digits = Digits::lookup_u8(self.0.g());
        let b_digits = Digits::lookup_u8(self.0.b());

        buffer.extend_from_slice(b"\x1b[48;2;");
        buffer.extend_from_slice(r_digits.ascii());
        buffer.push(b';');
        buffer.extend_from_slice(g_digits.ascii());
        buffer.push(b';');
        buffer.extend_from_slice(b_digits.ascii());
        buffer.push(b'm');
    }

    #[inline]
    fn length(self) -> usize {
        let r_digits = Digits::lookup_u8(self.0.r());
        let g_digits = Digits::lookup_u8(self.0.g());
        let b_digits = Digits::lookup_u8(self.0.b());

        10 + r_digits.count() + g_digits.count() + b_digits.count()
    }
}

pub struct Alternate<const TOGGLE: bool>;
impl Sequence for Alternate<true> {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(b"\x1b[?1049h");
    }

    #[inline]
    fn length(self) -> usize {
        8
    }
}
impl Sequence for Alternate<false> {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(b"\x1b[?1049l");
    }

    #[inline]
    fn length(self) -> usize {
        8
    }
}

pub struct Cursor<const TOGGLE: bool>;
impl Sequence for Cursor<true> {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(b"\x1b[?25h");
    }

    #[inline]
    fn length(self) -> usize {
        6
    }
}
impl Sequence for Cursor<false> {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(b"\x1b[?25l")
    }

    #[inline]
    fn length(self) -> usize {
        6
    }
}

pub struct Clear;
impl Sequence for Clear {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(b"\x1b[2J");
    }

    #[inline]
    fn length(self) -> usize {
        4
    }
}

pub struct Reset;
impl Sequence for Reset {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(b"\x1b[0m");
    }

    #[inline]
    fn length(self) -> usize {
        4
    }
}

pub struct Absolute(pub Position);
impl Sequence for Absolute {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        if self.0 == Position::ZERO {
            buffer.extend_from_slice(b"\x1b[H");
        } else {
            let x_digits = Digits::from_u16(self.0.x().saturating_add(1));
            let y_digits = Digits::from_u16(self.0.y().saturating_add(1));

            buffer.extend_from_slice(b"\x1b[");
            buffer.extend_from_slice(y_digits.ascii());
            buffer.push(b';');
            buffer.extend_from_slice(x_digits.ascii());
            buffer.push(b'H');
        }
    }

    #[inline]
    fn length(self) -> usize {
        if self.0 == Position::ZERO {
            3
        } else {
            let x_digits = Digits::from_u16(self.0.x().saturating_add(1));
            let y_digits = Digits::from_u16(self.0.y().saturating_add(1));

            4 + y_digits.count() + x_digits.count()
        }
    }
}

pub struct Column(pub u16);
impl Sequence for Column {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        if self.0 == 0 {
            buffer.extend_from_slice(b"\x1b[G");
        } else {
            let digits = Digits::from_u16(self.0.saturating_add(1));

            buffer.extend_from_slice(b"\x1b[");
            buffer.extend_from_slice(digits.ascii());
            buffer.push(b'G');
        }
    }

    #[inline]
    fn length(self) -> usize {
        if self.0 == 0 {
            3
        } else {
            3 + Digits::from_u16(self.0.saturating_add(1)).count()
        }
    }
}

pub struct Row(pub u16);
impl Sequence for Row {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        if self.0 == 0 {
            buffer.extend_from_slice(b"\x1b[d");
        } else {
            let digits = Digits::from_u16(self.0.saturating_add(1));

            buffer.extend_from_slice(b"\x1b[");
            buffer.extend_from_slice(digits.ascii());
            buffer.push(b'd');
        }
    }

    #[inline]
    fn length(self) -> usize {
        if self.0 == 0 {
            3
        } else {
            3 + Digits::from_u16(self.0.saturating_add(1)).count()
        }
    }
}

pub struct Up(pub u16);
impl Sequence for Up {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        if self.0 == 0 || self.0 == 1 {
            buffer.extend_from_slice(b"\x1b[A");
        } else {
            let digits = Digits::from_u16(self.0);

            buffer.extend_from_slice(b"\x1b[");
            buffer.extend_from_slice(digits.ascii());
            buffer.push(b'A');
        }
    }

    #[inline]
    fn length(self) -> usize {
        if self.0 == 0 || self.0 == 1 {
            3
        } else {
            3 + Digits::from_u16(self.0).count()
        }
    }
}

pub struct Down(pub u16);
impl Sequence for Down {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        if self.0 == 0 || self.0 == 1 {
            buffer.extend_from_slice(b"\x1b[B");
        } else {
            let digits = Digits::from_u16(self.0);

            buffer.extend_from_slice(b"\x1b[");
            buffer.extend_from_slice(digits.ascii());
            buffer.push(b'B');
        }
    }

    #[inline]
    fn length(self) -> usize {
        if self.0 == 0 || self.0 == 1 {
            3
        } else {
            3 + Digits::from_u16(self.0).count()
        }
    }
}

pub struct Right(pub u16);
impl Sequence for Right {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        if self.0 == 0 || self.0 == 1 {
            buffer.extend_from_slice(b"\x1b[C");
        } else {
            let digits = Digits::from_u16(self.0);

            buffer.extend_from_slice(b"\x1b[");
            buffer.extend_from_slice(digits.ascii());
            buffer.push(b'C');
        }
    }

    #[inline]
    fn length(self) -> usize {
        if self.0 == 0 || self.0 == 1 {
            3
        } else {
            3 + Digits::from_u16(self.0).count()
        }
    }
}

pub struct Left(pub u16);
impl Sequence for Left {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        if self.0 == 0 || self.0 == 1 {
            buffer.extend_from_slice(b"\x1b[D");
        } else {
            let digits = Digits::from_u16(self.0);

            buffer.extend_from_slice(b"\x1b[");
            buffer.extend_from_slice(digits.ascii());
            buffer.push(b'D');
        }
    }

    #[inline]
    fn length(self) -> usize {
        if self.0 == 0 || self.0 == 1 {
            3
        } else {
            3 + Digits::from_u16(self.0).count()
        }
    }
}
