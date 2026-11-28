use std::time::{Duration, Instant};

#[must_use = "measurement ends immediately if not bound to a variable"]
pub struct Sample<'a>(&'a mut Watch, Instant);
impl Drop for Sample<'_> {
    #[inline]
    fn drop(&mut self) {
        self.0.elapsed += self.1.elapsed();
        self.0.laps += 1;
    }
}

#[derive(Clone, Copy)]
pub struct Watch {
    elapsed: Duration,
    laps: u32,
}

impl Watch {
    #[inline]
    pub fn new() -> Self {
        let laps = 0;
        let elapsed = Duration::ZERO;
        Self { elapsed, laps }
    }

    #[inline]
    pub fn sample(&mut self) -> Sample<'_> {
        Sample(self, Instant::now())
    }

    #[inline]
    pub fn total(&self) -> Duration {
        self.elapsed
    }

    #[inline]
    pub fn average(&self) -> Duration {
        self.elapsed / self.laps.max(1)
    }
}

impl Default for Watch {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
