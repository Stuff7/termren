use rand::Rng;
use super::console;

const PX: [char; 2] = ['█', '█'];

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
  texture: [char; 2],
  pub color: Color,
  pub x: u16,
  pub y: u16,
}

impl Pixel {
  pub fn randomize_position(&mut self, console_w: u16, console_h: u16) -> &mut Self {
    self.x = rand::thread_rng().gen_range(0..console_w);
    self.y = rand::thread_rng().gen_range(0..console_h);
    self
  }
}

impl std::fmt::Display for Pixel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let [px1, px2] = self.texture;
    write!(
      f, "{pos}{color}{px1}{px2}{reset}",
      pos = console::seq::goto(self.x, self.y),
      color = console::seq::fg_rgb(self.color.r, self.color.g, self.color.b),
      reset = console::seq::RESET,
    )
  }
}

impl PartialEq for Pixel {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }

  fn ne(&self, other: &Self) -> bool {
    self.x != other.x || self.y != other.y
  }
}

impl From<(u16, u16)> for Pixel {
  fn from(pos: (u16, u16)) -> Self {
    let mut rng = rand::thread_rng();
    Self {
      texture: PX,
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

impl From<(u16, u16, Color, [char; 2])> for Pixel {
  fn from(px: (u16, u16, Color, [char; 2])) -> Self {
    Self {
      texture: px.3,
      color: px.2,
      x: px.0,
      y: px.1,
    }
  }
}

impl From<(u16, u16, Color, char)> for Pixel {
  fn from(px: (u16, u16, Color, char)) -> Self {
    Self {
      texture: [px.3, px.3],
      color: px.2,
      x: px.0,
      y: px.1,
    }
  }
}

impl From<(u16, u16, Color)> for Pixel {
  fn from(px: (u16, u16, Color)) -> Self {
    Self {
      texture: PX,
      color: px.2,
      x: px.0,
      y: px.1,
    }
  }
}

impl From<(u16, u16, u8, u8, u8)> for Pixel {
  fn from(px: (u16, u16, u8, u8, u8)) -> Self {
    Self {
      texture: PX,
      color: (px.2, px.3, px.4).into(),
      x: px.0,
      y: px.1,
    }
  }
}

#[derive(Clone, Copy)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Color {
  pub const fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }

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
