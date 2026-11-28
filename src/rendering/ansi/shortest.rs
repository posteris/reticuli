use crate::{
    common::position::Position,
    rendering::ansi::sequence::{Absolute, Column, Down, Left, Right, Row, Sequence, Up},
};

pub struct Shortest(pub Option<Position>, pub Position);
impl Sequence for Shortest {
    #[inline]
    fn encode(self, buffer: &mut Vec<u8>) {
        let to = self.1;
        let Some(from) = self.0 else {
            Absolute(to).encode(buffer);
            return;
        };

        if from == to {
            return;
        }

        let vertical = vertical_length(from.y(), to.y());
        let horizontal = horizontal_length(from.x(), to.x());

        if vertical + horizontal < Absolute(to).length() {
            encode_vertical(buffer, from.y(), to.y());
            encode_horizontal(buffer, from.x(), to.x());
        } else {
            Absolute(to).encode(buffer);
        }
    }

    #[inline]
    fn length(self) -> usize {
        todo!()
    }
}

#[inline]
fn vertical_length(from: u16, to: u16) -> usize {
    if from == to {
        return 0;
    }

    let distance = to.abs_diff(from);
    let relative = Up(distance).length();
    let absolute = Row(to).length();

    if relative <= absolute {
        relative
    } else {
        absolute
    }
}

#[inline]
fn encode_vertical(buffer: &mut Vec<u8>, from: u16, to: u16) {
    if from == to {
        return;
    }

    let distance = to.abs_diff(from);
    let relative = Up(distance).length();
    let absolute = Row(to).length();

    if relative <= absolute {
        if to < from {
            Up(distance).encode(buffer)
        } else {
            Down(distance).encode(buffer)
        }
    } else {
        Row(to).encode(buffer)
    }
}

#[inline]
fn horizontal_length(from: u16, to: u16) -> usize {
    if from == to {
        return 0;
    }

    let distance = to.abs_diff(from);
    let relative = Left(distance).length();
    let absolute = Column(to).length();

    if relative <= absolute {
        relative
    } else {
        absolute
    }
}

#[inline]
fn encode_horizontal(buffer: &mut Vec<u8>, from: u16, to: u16) {
    if from == to {
        return;
    }

    let distance = to.abs_diff(from);
    let relative = Left(distance).length();
    let absolute = Column(to).length();

    if relative <= absolute {
        if to < from {
            Left(distance).encode(buffer)
        } else {
            Right(distance).encode(buffer)
        }
    } else {
        Column(to).encode(buffer)
    }
}
