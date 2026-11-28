use std::time::Instant;

use crate::{
    common::{cell::Cell, color::Color},
    rendering::{canvas::Canvas, painter::Painter},
};

pub struct Chaos {
    origin: Instant,
}

impl Chaos {
    #[inline]
    pub fn new() -> Self {
        let origin = Instant::now();
        Self { origin }
    }
}

impl Default for Chaos {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Painter for Chaos {
    fn paint(&mut self, canvas: &mut Canvas) {
        let seed = self.origin.elapsed().as_micros() as u32;

        canvas.fill_with(|position| {
            let (xh, yh) = (u32::from(position.x()), u32::from(position.y()));
            let mut h = xh
                .wrapping_mul(374_761_393)
                .wrapping_add(yh.wrapping_mul(668_265_263))
                .wrapping_add(seed.wrapping_mul(2_246_822_519));
            h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
            h ^= h >> 16;

            let color = Color::new(h as u8, (h >> 8) as u8, (h >> 16) as u8);
            Cell::solid(color)
        });
    }
}
