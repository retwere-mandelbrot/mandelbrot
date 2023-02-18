#[macro_use]
extern crate rocket;

use mandelbrot::{render_to_file, Axis, ComplexPlane, Mandelbrot, Plane};
use rocket::fs::NamedFile;
use std::fs;
use std::path::Path;

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
const DOMAIN: Plane = Plane { x: X, y: Y };
const CX: ComplexPlane = ComplexPlane { coords: DOMAIN };
const C: (f64, f64) = (0.0, 0.0);
const MANDELBROT: Mandelbrot = Mandelbrot {
  cx: CX,
  c: num_complex::Complex::new(C.0, C.1),
};

#[get("/<zoom>/<x>/<y>")]
async fn tilegen(zoom: u32, x: i64, y: i64) -> NamedFile {
  let dir = format!(".cache/{}/{}/", zoom, x);
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

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/tilegen", routes![tilegen])
}
