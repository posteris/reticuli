use crate::rendering::painter::Painter;
use std::io::Result;

pub trait Renderer {
    fn events(&mut self) -> Result<()>;
    fn paint(&mut self, painter: &mut impl Painter);
    fn present(&mut self);
    fn flush(&mut self) -> Result<()>;

    fn run(&mut self, painter: &mut impl Painter) -> Result<()> {
        loop {
            self.events()?;
            self.paint(painter);
            self.present();
            self.flush()?;
        }
    }
}
