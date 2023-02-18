use crate::{ComplexPlane, Fractal};

pub struct Julia {
  pub cx: ComplexPlane,
  pub c: num_complex::Complex64,
}

impl Fractal for Julia {
  fn sample(&self, p: (u32, u32)) -> u8 {
    let mut z = self.cx.sample(p);
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.c;
      i += 1;
    }
    i
  }

  fn res(&self) -> (u32, u32) {
    self.cx.res()
  }
}
