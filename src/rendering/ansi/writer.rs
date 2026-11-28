use crate::common::color::Color;
use crate::common::position::Position;
use crate::common::size::Size;
use crate::rendering::ansi::sequence::{self, Sequence};
use crate::rendering::ansi::shortest::Shortest;

pub struct Writer {
    size: Size,
    buffer: Vec<u8>,
    pending_wrap: bool,
    cursor: Option<Position>,
    foreground: Option<Color>,
    background: Option<Color>,
}

impl Writer {
    #[inline]
    pub fn new(size: Size) -> Self {
        Self {
            size,
            buffer: Vec::with_capacity(size.area() as usize * 64),
            pending_wrap: false,
            cursor: None,
            foreground: None,
            background: None,
        }
    }

    #[inline]
    pub fn written(&self) -> &[u8] {
        &self.buffer
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    #[inline]
    pub fn begin(&mut self) {
        self.clear();
        sequence::Cursor::<false>.encode(&mut self.buffer);
        sequence::Alternate::<true>.encode(&mut self.buffer);
        sequence::Clear.encode(&mut self.buffer);
    }

    #[inline]
    pub fn end(&mut self) {
        self.clear();
        sequence::Reset.encode(&mut self.buffer);
        sequence::Cursor::<true>.encode(&mut self.buffer);
        sequence::Alternate::<false>.encode(&mut self.buffer);
    }

    #[inline]
    pub fn forget(&mut self) {
        self.cursor = None;
        self.pending_wrap = false;
        self.foreground = None;
        self.background = None;
    }

    #[inline]
    pub fn resize(&mut self, size: Size) {
        self.clear();
        self.forget();
        sequence::Reset.encode(&mut self.buffer);
        sequence::Clear.encode(&mut self.buffer);
        self.size = size;
    }

    #[inline]
    pub fn emit_move(&mut self, target: Position) {
        let Some(cursor) = self.cursor else {
            Shortest(None, target).encode(&mut self.buffer);
            self.cursor = Some(target);
            self.pending_wrap = false;
            return;
        };

        let wrapped_position = cursor.with_x(0).add_y(1);
        if self.pending_wrap && target == wrapped_position {
            return;
        }

        Shortest(Some(cursor), target).encode(&mut self.buffer);

        if self.pending_wrap && cursor == target {
            sequence::Absolute(target).encode(&mut self.buffer);
        }

        self.cursor = Some(target);
        self.pending_wrap = false;
    }

    #[inline]
    pub fn emit_foreground(&mut self, color: Color) {
        if self.foreground != Some(color) {
            sequence::Foreground(color).encode(&mut self.buffer);
            self.foreground = Some(color);
        }
    }

    #[inline]
    pub fn emit_background(&mut self, color: Color) {
        if self.background != Some(color) {
            sequence::Background(color).encode(&mut self.buffer);
            self.background = Some(color);
        }
    }

    #[inline]
    pub fn emit_glyph_u8(&mut self, glyph: u8) {
        self.buffer.push(glyph);
        self.advance();
    }

    #[inline]
    fn advance(&mut self) {
        let Some(cursor) = self.cursor else {
            return;
        };

        let written = if self.pending_wrap {
            cursor.with_x(0).add_y(1)
        } else {
            cursor
        };

        let last = self.size.last();
        if written.x() < last.x() {
            self.cursor = Some(written.add_x(1));
            self.pending_wrap = false;
        } else if written.y() < last.y() {
            self.cursor = Some(written);
            self.pending_wrap = true;
        } else {
            self.cursor = Some(last);
            self.pending_wrap = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn size() -> Size {
        Size::new(4, 2)
    }

    fn writer() -> Writer {
        Writer::new(size())
    }

    fn write_at(writer: &mut Writer, target: Position, glyph: u8) {
        writer.emit_move(target);
        writer.emit_glyph_u8(glyph);
    }

    #[test]
    fn first_move_is_absolute() {
        let mut writer = writer();
        writer.emit_move(Position::new(1, 1));
        assert_eq!(writer.written(), b"\x1b[2;2H");
    }

    #[test]
    fn first_move_to_origin_is_home() {
        let mut writer = writer();
        writer.emit_move(Position::ZERO);
        assert_eq!(writer.written(), b"\x1b[H");
    }

    #[test]
    fn sequential_writes_need_no_movement() {
        let mut writer = writer();
        write_at(&mut writer, Position::ZERO, b'a');
        writer.clear();
        write_at(&mut writer, Position::new(1, 0), b'b');
        assert_eq!(writer.written(), b"b");
    }

    #[test]
    fn writes_advance_one_column() {
        let mut writer = writer();
        write_at(&mut writer, Position::ZERO, b'a');
        writer.clear();
        writer.emit_move(Position::new(1, 0));
        assert!(writer.written().is_empty());
    }

    #[test]
    fn wrapping_to_the_next_row_needs_no_movement() {
        let mut writer = writer();
        write_at(&mut writer, Position::new(3, 0), b'a');
        writer.clear();
        write_at(&mut writer, Position::new(0, 1), b'b');
        assert_eq!(writer.written(), b"b");
    }

    #[test]
    fn moving_after_a_pending_wrap_uses_the_physical_cursor() {
        let mut writer = writer();
        write_at(&mut writer, Position::new(3, 0), b'a');
        writer.clear();
        writer.emit_move(Position::new(1, 0));
        assert_eq!(writer.written(), b"\x1b[2D");
    }

    #[test]
    fn moving_to_the_wrapped_position_cancels_the_wrap() {
        let mut writer = writer();
        write_at(&mut writer, Position::new(3, 0), b'a');
        writer.clear();
        writer.emit_move(Position::new(3, 0));
        assert_eq!(writer.written(), b"\x1b[1;4H");
    }

    #[test]
    fn last_cell_does_not_set_pending_wrap() {
        let mut writer = writer();
        write_at(&mut writer, Position::new(3, 1), b'a');
        writer.clear();
        writer.emit_move(Position::new(3, 1));
        assert!(writer.written().is_empty());
    }

    #[test]
    fn forgetting_forces_an_absolute_move() {
        let mut writer = writer();
        write_at(&mut writer, Position::ZERO, b'a');
        writer.forget();
        writer.clear();
        writer.emit_move(Position::new(1, 0));
        assert_eq!(writer.written(), b"\x1b[1;2H");
    }

    #[test]
    fn resizing_forgets_the_cursor() {
        let mut writer = writer();
        write_at(&mut writer, Position::ZERO, b'a');
        writer.resize(size());
        writer.clear();
        writer.emit_move(Position::new(1, 0));
        assert_eq!(writer.written(), b"\x1b[1;2H");
    }

    #[test]
    fn repeated_colors_are_not_re_emitted() {
        let mut writer = writer();
        writer.emit_foreground(Color::BLACK);
        writer.emit_foreground(Color::BLACK);
        writer.emit_background(Color::BLACK);
        writer.emit_background(Color::BLACK);
        assert_eq!(writer.written(), b"\x1b[38;2;0;0;0m\x1b[48;2;0;0;0m");
    }

    #[test]
    fn begin_writes_startup_controls() {
        let mut writer = writer();
        writer.begin();
        assert_eq!(writer.written(), b"\x1b[?25l\x1b[?1049h\x1b[2J");
    }

    #[test]
    fn end_writes_shutdown_controls() {
        let mut writer = writer();
        writer.end();
        assert_eq!(writer.written(), b"\x1b[0m\x1b[?25h\x1b[?1049l");
    }

    #[test]
    fn resize_writes_reset_and_clear() {
        let mut writer = writer();
        writer.resize(size());
        assert_eq!(writer.written(), b"\x1b[0m\x1b[2J");
    }
}
