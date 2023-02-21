#[macro_use]
extern crate rocket;

use mandelbrot::{render_to_file, Axis, ComplexPlane, Julia, Mandelbrot, Plane, Tile};
use rocket::fs::NamedFile;
use std::fs;
use std::path::Path;

// Coordinates.
const RESOLUTION: u32 = 512;
const X: Axis = Axis {
  min: -4.0,
  max: 4.0,
  res: RESOLUTION,
};
const Y: Axis = Axis {
  min: -4.0,
  max: 4.0,
  res: RESOLUTION,
};
const COORDS: Plane = Plane { x: X, y: Y };
const DOMAIN: ComplexPlane = ComplexPlane { coords: COORDS };

// Mandelbrot fractal
const C: (f64, f64) = (0.0, 0.0);
const MANDELBROT: Mandelbrot = Mandelbrot {
  cx: DOMAIN,
  c: num_complex::Complex::new(C.0, C.1),
};

const MANDELBROT_CACHE_DIR: &str = ".cache/";
const JULIA_CACHE_DIR: &str = ".cache/julia/";

#[get("/<zoom>/<x>/<y>")]
async fn tile_mandelbrot(zoom: u32, x: i64, y: i64) -> NamedFile {
  let dir = format!("{}/{}/{}/", MANDELBROT_CACHE_DIR, zoom, x);
  if !Path::exists(Path::new(&dir)) {
    fs::create_dir_all(&dir).unwrap();
  }
  let filename = format!("{}/{}.png", dir, y);
  if !Path::exists(Path::new(&filename)) {
    let fract = MANDELBROT.tile(zoom, (x, y));
    render_to_file(&fract, &filename).unwrap();
  }
  NamedFile::open(&filename).await.unwrap()
}

#[get("/<cx>/<cy>/<zoom>/<x>/<y>")]
async fn tile_julia(cx: f64, cy: f64, zoom: u32, x: i64, y: i64) -> NamedFile {
  let dir = format!("{}/{},{}/{}/{}/", JULIA_CACHE_DIR, cx, cy, zoom, x);
  if !Path::exists(Path::new(&dir)) {
    fs::create_dir_all(&dir).unwrap();
  }
  let filename = format!("{}/{}.png", dir, y);
  if !Path::exists(Path::new(&filename)) {
    let base: Julia = Julia {
      cx: DOMAIN,
      c: num_complex::Complex::new(cx, cy),
    };
    let fract = base.tile(zoom, (x, y));
    render_to_file(&fract, &filename).unwrap();
  }
  NamedFile::open(&filename).await.unwrap()
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/mandelbrot", routes![tile_mandelbrot])
    .mount("/julia", routes![tile_julia])
}
