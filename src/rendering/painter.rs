use crate::rendering::canvas::Canvas;

pub trait Painter {
    fn paint(&mut self, canvas: &mut Canvas);
}
