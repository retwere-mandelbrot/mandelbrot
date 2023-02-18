pub trait Fractal {
  fn sample(&self, p: (u32, u32)) -> u8;
  fn res(&self) -> (u32, u32);
}

mod julia;
pub use julia::Julia;

mod mandelbrot;
pub use mandelbrot::Mandelbrot;
