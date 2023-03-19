use fractal::{Axis, ComplexPlane, Fractal, Julia, Mandelbrot, Plane, Tile};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

mod render;
use render::render;

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

const C: (f64, f64) = (0.0, 0.0);
const MANDELBROT: Mandelbrot = Mandelbrot {
  cx: DOMAIN,
  c: num_complex::Complex::new(C.0, C.1),
};

#[wasm_bindgen]
pub fn render_mandelbrot(
  zoom: u32,
  x: i64,
  y: i64,
  context: &CanvasRenderingContext2d,
) -> Result<(), JsValue> {
  let fract = MANDELBROT.tile(zoom, (x, y));
  let res = fract.res();
  let mut raw = render(&fract);
  let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw), res.0, res.1)?;
  context.put_image_data(&data, 0.0, 0.0)
}

#[wasm_bindgen]
pub fn render_julia(
  cx: f64,
  cy: f64,
  zoom: u32,
  x: i64,
  y: i64,
  context: &CanvasRenderingContext2d,
) -> Result<(), JsValue> {
  let base: Julia = Julia {
    cx: DOMAIN,
    c: num_complex::Complex::new(cx, cy),
  };
  let fract = base.tile(zoom, (x, y));
  let res = fract.res();
  let mut raw = render(&fract);
  let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw), res.0, res.1)?;
  context.put_image_data(&data, 0.0, 0.0)
}
