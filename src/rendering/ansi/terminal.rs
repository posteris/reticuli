use crate::rendering::ansi::platform::{raw_stdout, terminal_size};
use crate::rendering::ansi::writer::Writer;
use crate::rendering::canvas::Canvas;
use crate::rendering::painter::Painter;
use crate::rendering::renderer::Renderer;
use std::fs::File;
use std::io::{Result, Write};

pub struct Terminal {
    canvas: Canvas,
    writer: Writer,
    output: File,
}

impl Terminal {
    #[inline]
    pub fn new() -> Result<Self> {
        let mut output = raw_stdout()?;
        let size = terminal_size(&output)?;

        let canvas = Canvas::new(size);
        let mut writer = Writer::new(size);
        writer.begin();

        output.write_all(writer.written())?;
        writer.clear();

        Ok(Self {
            canvas,
            writer,
            output,
        })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.writer.end();
        let _ = self.output.write_all(self.writer.written());
    }
}

impl Renderer for Terminal {
    #[inline]
    fn events(&mut self) -> Result<()> {
        let size = terminal_size(&self.output)?;
        if self.canvas.size() != size {
            self.canvas.resize(size);
            self.writer.resize(size);
        }

        Ok(())
    }

    #[inline]
    fn paint(&mut self, painter: &mut impl Painter) {
        painter.paint(&mut self.canvas);
    }

    #[inline]
    fn present(&mut self) {
        for delta in self.canvas.delta() {
            self.writer.emit_move(delta.position());
            self.writer.emit_foreground(delta.cell().foreground());
            self.writer.emit_background(delta.cell().background());
            self.writer.emit_glyph_u8(delta.cell().glyph());
        }
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        if let Err(error) = self.output.write_all(self.writer.written()) {
            self.writer.forget();
            self.writer.clear();
            return Err(error);
        }

        self.writer.clear();
        self.canvas.clear_delta();
        Ok(())
    }
}
