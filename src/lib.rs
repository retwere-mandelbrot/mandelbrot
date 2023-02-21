mod coords;
pub use coords::{Axis, ComplexPlane, Plane};

mod fractal;
pub use fractal::{Fractal, Julia, Mandelbrot, Tile};

mod render;
pub use render::{render, render_to_file};
