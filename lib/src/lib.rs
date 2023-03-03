pub trait Fractal {
  fn sample(&self, p: (u32, u32)) -> u8;
  fn res(&self) -> (u32, u32);
}

pub trait Tile {
  fn tile(&self, zoom: u32, p: (i64, i64)) -> Self;
}

mod julia;
pub use julia::Julia;

mod mandelbrot;
pub use mandelbrot::Mandelbrot;

mod coords;
pub use coords::{Axis, ComplexPlane, Plane};

mod render;
pub use render::render;
