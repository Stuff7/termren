use rand::Rng;
use super::console;

const PX: &str = "██";

pub struct Drawable {
  pub pixels: Vec<Pixel>,
}

impl std::fmt::Display for Drawable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut pixels: Vec<String> = Vec::with_capacity(self.pixels.len());
    for px in self.pixels.iter() {
      pixels.push(px.to_string());
    }
    write!(f, "{}", pixels.join(""))
  }
}

impl From<Vec<Pixel>> for Drawable {
  fn from(pixels: Vec<Pixel>) -> Self {
    Self { pixels }
  }
}

pub struct Pixel {
  pub color: Color,
  pub x: u16,
  pub y: u16,
}

impl std::fmt::Display for Pixel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f, "{pos}{color}{PX}{reset}",
      pos = console::seq::goto(self.x, self.y),
      color = console::seq::fg_rgb(self.color.r, self.color.g, self.color.b),
      reset = console::seq::RESET,
    )
  }
}

impl From<(u16, u16)> for Pixel {
  fn from(pos: (u16, u16)) -> Self {
    let mut rng = rand::thread_rng();
    Self {
      color: (
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
      ).into(),
      x: pos.0,
      y: pos.1,
    }
  }
}

impl From<(u16, u16, u8, u8, u8)> for Pixel {
  fn from(px: (u16, u16, u8, u8, u8)) -> Self {
    Self {
      color: (px.2, px.3, px.4).into(),
      x: px.0,
      y: px.1,
    }
  }
}

pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Color {
  pub fn randomize(&mut self) -> &mut Self {
    let mut rng = rand::thread_rng();
    self.r = rng.gen::<u8>();
    self.g = rng.gen::<u8>();
    self.b = rng.gen::<u8>();
    self
  }
}

impl From<(u8, u8, u8)> for Color {
  fn from(color: (u8, u8, u8)) -> Self {
    Self {
      r: color.0,
      g: color.1,
      b: color.2,
    }
  }
}