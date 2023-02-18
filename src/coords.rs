pub struct Axis {
  pub min: f64,
  pub max: f64,
  pub res: u32,
}

impl Axis {
  pub fn scale(&self) -> f64 {
    (self.max - self.min) / self.res as f64
  }

  // Yields the `i`th sample in the coordinate range
  pub fn sample(&self, i: u32) -> f64 {
    i as f64 * self.scale() + self.min
  }

  pub fn res(&self) -> u32 {
    self.res
  }

  pub fn tile(&self, zoom: u32, i: i64) -> Axis {
    // At zoom 0, each tile covers 8.0 units. Range doubles for each zoom level.
    let range: f64 = 8.0 / (2.0f64).powf(zoom as f64);
    Axis {
      min: self.min + range * (i as f64),
      max: self.min + range * (1.0 + i as f64),
      res: self.res,
    }
  }
}

pub struct Plane {
  pub x: Axis,
  pub y: Axis,
}

impl Plane {
  pub fn sample(&self, p: (u32, u32)) -> (f64, f64) {
    (self.x.sample(p.0), self.y.sample(p.1))
  }

  pub fn res(&self) -> (u32, u32) {
    (self.x.res(), self.y.res())
  }

  pub fn tile(&self, zoom: u32, p: (i64, i64)) -> Plane {
    Plane {
      x: self.x.tile(zoom, p.0),
      y: self.y.tile(zoom, p.1),
    }
  }
}

// The complex plane.
pub struct ComplexPlane {
  pub coords: Plane,
}

impl ComplexPlane {
  pub fn sample(&self, p: (u32, u32)) -> num_complex::Complex64 {
    let p = self.coords.sample(p);
    num_complex::Complex::new(p.0, p.1)
  }

  pub fn res(&self) -> (u32, u32) {
    self.coords.res()
  }

  pub fn tile(&self, zoom: u32, p: (i64, i64)) -> ComplexPlane {
    ComplexPlane {
      coords: self.coords.tile(zoom, p),
    }
  }
}
