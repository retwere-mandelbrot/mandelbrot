use crate::ComplexPlane;
use crate::Fractal;

pub struct Mandelbrot {
  pub cx: ComplexPlane,
  pub c: num_complex::Complex64,
}

impl Fractal for Mandelbrot {
  fn sample(&self, p: (u32, u32)) -> u8 {
    let mut z = self.c;
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.cx.sample(p);
      i += 1;
    }
    i
  }

  fn res(&self) -> (u32, u32) {
    self.cx.res()
  }
}

impl Mandelbrot {
  pub fn tile(&self, zoom: u32, p: (i64, i64)) -> Mandelbrot {
    Mandelbrot {
      cx: self.cx.tile(zoom, p),
      c: self.c,
    }
  }
}
