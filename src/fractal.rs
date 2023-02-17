use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Rgb};

pub struct Axis {
  pub min: f64,
  pub max: f64,
  pub res: u32,
}

// A coordinate axis.
impl Axis {
  pub fn new(min: f64, max: f64, res: u32) -> Self {
    Axis { min, max, res }
  }

  pub fn scale(&self) -> f64 {
    (self.max - self.min) / self.res as f64
  }

  pub fn sample(&self, i: u32) -> f64 {
    i as f64 * self.scale() + self.min
  }
}

// The coordinate plane.
pub struct Plane {
  pub x: Axis,
  pub y: Axis,
}

impl Plane {
  pub fn new(x_min: f64, x_max: f64, x_res: u32, y_min: f64, y_max: f64, y_res: u32) -> Self {
    Plane {
      x: Axis::new(x_min, x_max, x_res),
      y: Axis::new(y_min, y_max, y_res),
    }
  }

  pub fn sample(&self, p: (u32, u32)) -> (f64, f64) {
    (self.x.sample(p.0), self.y.sample(p.1))
  }
}

// The complex plane.
pub struct ComplexPlane {
  pub coords: Plane,
}

impl ComplexPlane {
  pub fn new(x_min: f64, x_max: f64, x_res: u32, y_min: f64, y_max: f64, y_res: u32) -> Self {
    ComplexPlane {
      coords: Plane::new(x_min, x_max, x_res, y_min, y_max, y_res),
    }
  }

  pub fn sample(&self, p: (u32, u32)) -> num_complex::Complex64 {
    let p = self.coords.sample(p);
    num_complex::Complex::new(p.0, p.1)
  }
}

pub trait FractalConstructor {
  fn new(domain: ComplexPlane, c: (f64, f64)) -> Self;
}

pub trait Fractal {
  fn sample(&self, p: (u32, u32)) -> u8;
}

pub struct Julia {
  pub cx: ComplexPlane,
  pub c: num_complex::Complex64,
}

impl FractalConstructor for Julia {
  fn new(domain: ComplexPlane, c: (f64, f64)) -> Self {
    Julia {
      cx: domain,
      c: num_complex::Complex::new(c.0, c.1),
    }
  }
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
}

pub struct Mandelbrot {
  pub cx: ComplexPlane,
  pub c: num_complex::Complex64,
}

impl FractalConstructor for Mandelbrot {
  fn new(domain: ComplexPlane, c: (f64, f64)) -> Self {
    Mandelbrot {
      cx: domain,
      c: num_complex::Complex::new(c.0, c.1),
    }
  }
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
}

pub fn render<Container>(fract: &dyn Fractal, imgbuf: &mut ImageBuffer<Rgb<u8>, Container>) -> ()
where
  Container: Deref<Target = [u8]>,
  Container: DerefMut<Target = [u8]>,
{
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let i = 255 - fract.sample((x, y));
    *pixel = Rgb([i, i, i]);
  }
}
