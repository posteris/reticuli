use crate::{
    diagnostics::{chaos::Chaos, watch::Watch},
    rendering::renderer::Renderer,
};
use std::{io::Result, time::Duration};

#[derive(Default)]
pub struct Profiler {
    events: Watch,
    paint: Watch,
    present: Watch,
    flush: Watch,
}

impl Profiler {
    #[inline]
    pub fn bench(&mut self, renderer: &mut impl Renderer, iterations: usize) -> Result<()> {
        let mut chaos = Chaos::new();

        for _ in 0..iterations {
            {
                let _sample = self.events.sample();
                renderer.events()?;
            }

            {
                let _sample = self.paint.sample();
                renderer.paint(&mut chaos);
            }

            {
                let _sample = self.present.sample();
                renderer.present();
            }

            {
                let _sample = self.flush.sample();
                renderer.flush()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn averages(&self) -> [(&'static str, Duration); 4] {
        [
            ("events", self.events.average()),
            ("paint", self.paint.average()),
            ("present", self.present.average()),
            ("flush", self.flush.average()),
        ]
    }
}
