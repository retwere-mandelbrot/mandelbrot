pub(crate) struct Coord {
  pub(crate) min: f64,
  pub(crate) max: f64,
  pub(crate) samples: u32,
}

impl Coord {
  pub(crate) fn scale(&self) -> f64 {
    (self.max - self.min) / self.samples as f64
  }

  pub(crate) fn sample(&self, i: u32) -> f64 {
    i as f64 * self.scale() + self.min
  }
}
