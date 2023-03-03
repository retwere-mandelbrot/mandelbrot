use fractal::Fractal;

pub fn render(fract: &dyn Fractal) -> Vec<u8> {
  let (x_res, y_res) = fract.res();
  let mut data = vec![0; (4 * x_res * y_res).try_into().unwrap()];
  let mut pxl = 0;
  for y in 0..y_res {
    for x in 0..x_res {
      let i = 255 - fract.sample((x, y));
      data[pxl] = i; // R
      data[pxl + 1] = i; // G
      data[pxl + 2] = i; // B
      data[pxl + 3] = 255; // A
      pxl = pxl + 4;
    }
  }
  data
}
