use std::ops::{Deref, DerefMut};

use crate::Fractal;
use image::{ImageBuffer, ImageError, ImageFormat, Rgb};

pub fn render<Container>(fract: &dyn Fractal, imgbuf: &mut ImageBuffer<Rgb<u8>, Container>) -> ()
where
  Container: Deref<Target = [u8]>,
  Container: DerefMut<Target = [u8]>,
{
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let i = 255 - fract.sample((x, y));
    *pixel = Rgb([i, i, i]);
  }
}

pub fn render_to_file(fract: &dyn Fractal, filename: &String) -> Result<(), ImageError> {
  let res = fract.res();
  let mut imgbuf = ImageBuffer::new(res.0, res.1);
  render(fract, &mut imgbuf);
  imgbuf.save_with_format(filename, ImageFormat::Png)
}
