struct Coord {
  min: f64,
  max: f64,
  samples: u32,
}

impl Coord {
  fn scale(&self) -> f64 {
    (self.max - self.min) / self.samples as f64
  }

  fn sample(&self, i: u32) -> f64 {
    i as f64 * self.scale() + self.min
  }
}

struct Fractal {
  x: Coord,
  y: Coord,
  c: num_complex::Complex64,
}

impl Fractal {
  fn z(&self, px: u32, py: u32) -> num_complex::Complex64 {
    num_complex::Complex::new(self.x.sample(px), self.y.sample(py))
  }

  fn julia(&self, x: u32, y: u32) -> u8 {
    let mut z = self.z(x, y);
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.c;
      i += 1;
    }
    i
  }

  fn mandelbrot(&self, x: u32, y: u32) -> u8 {
    let mut z = self.c;
    let mut i = 0;
    while i < 255 && z.norm() <= 2.0 {
      z = z * z + self.z(x, y);
      i += 1;
    }
    i
  }
}

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// generate a Julia set fractal instead of a Mandelbrot set.
  #[arg(short, long)]
  julia: bool,

  /// resolution for x coordinate.
  #[arg(long, default_value_t = 1024)]
  x_res: u32,

  /// resolution for y coordinate.
  #[arg(long, default_value_t = 1024)]
  y_res: u32,

  /// minimum for x coordinate.
  #[arg(long, default_value_t = -2.5, allow_hyphen_values = true)]
  x_min: f64,

  /// maximum for x coordinate.
  #[arg(long, default_value_t = 2.5, allow_hyphen_values = true)]
  x_max: f64,

  /// minimum for y coordinate.
  #[arg(long, default_value_t = -2.5, allow_hyphen_values = true)]
  y_min: f64,

  /// maximum for y coordinate.
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
