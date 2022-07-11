use rand::Rng;
use super::console;

const PX: &str = "██";

#[derive(Debug)]
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

impl From<(u16, u16, Color)> for Pixel {
  fn from(px: (u16, u16, Color)) -> Self {
    Self {
      color: px.2,
      x: px.0,
      y: px.1,
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

impl std::fmt::Debug for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "rgb({},{},{})", self.r, self.g, self.b)
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
