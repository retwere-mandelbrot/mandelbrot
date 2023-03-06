use fractal::{Axis, ComplexPlane, Fractal, Julia, Mandelbrot, Plane, Tile};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::HtmlCanvasElement;
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
pub fn mandelbrot_tile(zoom: u32, x: i64, y: i64) -> Result<HtmlCanvasElement, JsValue> {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document
    .create_element("canvas")?
    .dyn_into::<web_sys::HtmlCanvasElement>()?;
  let context = canvas
    .get_context("2d")?
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  render_mandelbrot(zoom, x, y, &context)?;
  Ok(canvas)
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

#[wasm_bindgen]
pub fn render_smiley(context: &CanvasRenderingContext2d) -> () {
  context.begin_path();

  // Draw the outer circle.
  context
    .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
    .unwrap();

  // Draw the mouth.
  context.move_to(110.0, 75.0);
  context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

  // Draw the left eye.
  context.move_to(65.0, 65.0);
  context
    .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    .unwrap();

  // Draw the right eye.
  context.move_to(95.0, 65.0);
  context
    .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    .unwrap();

  context.stroke();
}
