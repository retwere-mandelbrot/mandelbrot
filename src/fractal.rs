use crate::coord::Coord;

pub(crate) struct Fractal {
  pub(crate) x: Coord,
  pub(crate) y: Coord,
  pub(crate) c: num_complex::Complex64,
}

impl Fractal {
  fn z(&self, px: u32, py: u32) -> num_complex::Complex64 {
    num_complex::Complex::new(self.x.sample(px), self.y.sample(py))
  }

  pub(crate) fn julia(&self, x: u32, y: u32) -> u8 {
    let mut z = self.z(x, y);
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.c;
      i += 1;
    }
    i
  }

  pub(crate) fn mandelbrot(&self, x: u32, y: u32) -> u8 {
    let mut z = self.c;
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.z(x, y);
      i += 1;
    }
    i
  }
}
