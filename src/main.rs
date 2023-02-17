use clap::Parser;
use image::ImageFormat;
use mandelbrot::render;
use mandelbrot::ComplexPlane;
use mandelbrot::FractalConstructor;
use mandelbrot::Julia;
use mandelbrot::Mandelbrot;

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

  /// Output filename.
  #[arg(short, long, value_name = "FILE", default_value_t = String::from("fractal.png"))]
  output: String,
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
    output,
  } = Args::parse();

  let domain = ComplexPlane::new(x_min, x_max, x_res, y_min, y_max, y_res);
  let mut imgbuf =
    image::ImageBuffer::<image::Rgb<u8>, _>::new(domain.coords.x.res, domain.coords.y.res);

  if julia {
    let fract = Julia::new(domain, (cx, cy));
    render(&fract, &mut imgbuf);
  } else {
    let fract = Mandelbrot::new(domain, (cx, cy));
    render(&fract, &mut imgbuf);
  };

  // Save the image as "[output]", the format is deduced from the path
  imgbuf.save_with_format(output, ImageFormat::Png).unwrap();
}
