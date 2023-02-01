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

fn main() {

fn main() {
  // TODO: these should be args
  let x_res = 1024;
  let y_res = 1024;

  let x_min = -2.1;
  let x_max = 0.7;

  let y_min = -1.2;
  let y_max = 1.2;

  //let cx = -0.4;
  //let cy = 0.6;
  let cx = -0.0;
  let cy = 0.0;

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
    let i = 255 - fract.mandelbrot(x, y);
    *pixel = image::Rgb([i, i, i]);
  }

  // Save the image as “fractal.png”, the format is deduced from the path
  imgbuf.save("fractal.png").unwrap();
}
