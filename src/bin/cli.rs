use clap::Parser;
use mandelbrot::{render_to_file, Axis, ComplexPlane, Julia, Mandelbrot, Plane};

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

// A command-line interface for generating mandelbrot set PNG images.
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

  let x: Axis = Axis {
    min: x_min,
    max: x_max,
    res: x_res,
  };
  let y: Axis = Axis {
    min: y_min,
    max: y_max,
    res: y_res,
  };
  let coords: Plane = Plane { x, y };
  let domain: ComplexPlane = ComplexPlane { coords };
  if julia {
    let fract = Julia {
      cx: domain,
      c: num_complex::Complex::new(cx, cy),
    };
    render_to_file(&fract, &output).unwrap();
  } else {
    let fract = Mandelbrot {
      cx: domain,
      c: num_complex::Complex::new(cx, cy),
    };
    render_to_file(&fract, &output).unwrap();
  };
}
