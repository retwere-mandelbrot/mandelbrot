pub fn render(fract: &dyn Fractal) -> Vec<u8>
{
  let (x_res, y_res) = fract.res();
  let mut data = vec![0; (4 * x_res * y_res).try_into().unwrap()];
  let mut pxl = 0;
  for x in 0..x_res {
    for y in 0..y_res {
      let i = 255 - fract.sample((x, y));
      imgbuf[pxl] = i; // R
      imgbuf[pxl + 1] = i; // G
      imgbuf[pxl + 2] = i; // B
      imgbuf[pxl + 3] = 255; // A
      pxl = pxl + 4;
    }
  }
}
