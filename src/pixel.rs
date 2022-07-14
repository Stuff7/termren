use rand::Rng;
use super::console;

const PX: &str = "██";

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
  texture: &'static str,
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

  pub fn to_string(&self) -> String {
    let pos = console::seq::goto(self.x, self.y);
    let color = console::seq::fg_rgb(self.color.r, self.color.g, self.color.b);
    let mut px = String::with_capacity(
      pos.len() + color.len() + self.texture.len() + console::seq::RESET.len() + 1
    );
    px.push_str(&pos);
    px.push_str(&color);
    px.push_str(self.texture);
    px.push_str(console::seq::RESET);
    px
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

impl From<(u16, u16, Color, &'static str)> for Pixel {
  fn from(px: (u16, u16, Color, &'static str)) -> Self {
    if px.3.chars().count() > 2 {
      panic!("Pixel texture can't be more than 2 characters long!");
    }
    Self {
      texture: px.3,
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
