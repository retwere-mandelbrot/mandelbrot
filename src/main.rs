mod coord;
mod fractal;

use crate::coord::Coord;
use crate::fractal::Fractal;
use clap::Parser;

/// Generates images of Mandelbrot sets and Julia sets.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Generate a Julia set fractal instead of a Mandelbrot set.
  #[arg(short, long)]
  julia: bool,

  /// Resolution for x coordinate.
  #[arg(long, default_value_t = 1024)]
  x_res: u32,

  /// Resolution for y coordinate.
  #[arg(long, default_value_t = 1024)]
  y_res: u32,

  /// Minimum for x coordinate.
  #[arg(long, default_value_t = -2.5, allow_hyphen_values = true)]
  x_min: f64,

  /// Maximum for x coordinate.
  #[arg(long, default_value_t = 2.5, allow_hyphen_values = true)]
  x_max: f64,

  /// Minimum for y coordinate.
  #[arg(long, default_value_t = -2.5, allow_hyphen_values = true)]
  y_min: f64,

  /// Maximum for y coordinate.
  #[arg(long, default_value_t = 2.5, allow_hyphen_values = true)]
  y_max: f64,

  /// x coordinate of constant c.
  #[arg(long, default_value_t = 0.0, allow_hyphen_values = true)]
  cx: f64,

  /// y coordinate of constant c.
  #[arg(long, default_value_t = 0.0, allow_hyphen_values = true)]
  cy: f64,
}

fn main() {
  let Args {
    julia,
    x_res,
    y_res,
    x_min,
    x_max,
    y_min,
    y_max,
    cx,
    cy,
  } = Args::parse();

  let fract = Fractal {
    x: Coord {
      min: x_min,
      max: x_max,
      samples: x_res,
    },
    y: Coord {
      min: y_min,
      max: y_max,
      samples: y_res,
    },
    c: num_complex::Complex::new(cx, cy),
  };

  // Create a new ImgBuf with width: imgx and height: imgy
  let mut imgbuf = image::ImageBuffer::new(fract.x.samples, fract.y.samples);

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let f = if julia {
      Fractal::julia
    } else {
      Fractal::mandelbrot
    };
    let i = 255 - f(&fract, x, y);
    *pixel = image::Rgb([i, i, i]);
  }

  // Save the image as “fractal.png”, the format is deduced from the path
  imgbuf.save("fractal.png").unwrap();
}
