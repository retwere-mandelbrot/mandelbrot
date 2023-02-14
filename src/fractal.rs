pub struct Coord {
  pub min: f64,
  pub max: f64,
  pub samples: u32,
}

impl Coord {
  pub fn scale(&self) -> f64 {
    (self.max - self.min) / self.samples as f64
  }

  pub fn sample(&self, i: u32) -> f64 {
    i as f64 * self.scale() + self.min
  }
}

pub struct Fractal {
  pub x: Coord,
  pub y: Coord,
  pub c: num_complex::Complex64,
}

impl Fractal {
  pub fn z(&self, px: u32, py: u32) -> num_complex::Complex64 {
    num_complex::Complex::new(self.x.sample(px), self.y.sample(py))
  }

  pub fn julia(&self, x: u32, y: u32) -> u8 {
    let mut z = self.z(x, y);
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.c;
      i += 1;
    }
    i
  }

  pub fn mandelbrot(&self, x: u32, y: u32) -> u8 {
    let mut z = self.c;
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.z(x, y);
      i += 1;
    }
    i
  }
}
