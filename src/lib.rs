mod fractal;
pub use fractal::{Axis, ComplexPlane, Fractal, Julia, Mandelbrot, Plane, Tile};

mod render;
pub use render::{render, render_to_file};
