pub use console;
mod pixel;

use std::{rc::Rc, cell::RefCell, time::Duration};
pub use pixel::{Pixel, Color};
use crossterm::event::{poll, read};
pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Renderer {
  ctx: Context,
  fps_capper: FPSCapper,
  handler: Rc<RefCell<dyn EventHandler>>,
}

impl Renderer {
  pub fn new(handler: Rc<RefCell<dyn EventHandler>>) -> Self {
    Self { ctx: Context::new(), fps_capper: FPSCapper::new(0), handler }
  }

  pub fn run(&mut self) {
    if let Some((w, h)) = console::size() {
      self.ctx.console_size = (w, h).into();
      console::clear();
      let exit_event = Event::Key(KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::CONTROL,
      });
      let empty_scene = " ".repeat(w.into()).repeat(h.into());
      let pos_last = console::seq::goto(1, h);
      let green = console::seq::fg_rgb(150, 255, 120);
      loop {
        let timer = std::time::Instant::now();
        let mut handler_borrow = self.handler.borrow_mut();
        let pixels: Group<Pixel>;
        let ui;
        self.ctx.timedelta = if self.fps_capper.fps > 0 {
          1.0 / self.fps_capper.fps as f64
        } else {0.0};
        if let Ok(true) = poll(Duration::from_millis(0)) {
          let event = read().unwrap();
          if event == exit_event {
            console::reset();
            console::show_cursor();
            console::goto(1, h);
            println!("\nExiting...");
            break
          }
          self.ctx.event = Some(event);
          (pixels, ui) = handler_borrow.update(&self.ctx);
        } else {
          self.ctx.event = None;
          (pixels, ui) = handler_borrow.update(&self.ctx);
        }

        let draws = match pixels {
          Group::Single(px) => {
            px.to_string()
          },
          Group::Multi(pxs) => {
            pxs.iter().map(|px| px.to_string()).collect::<String>()
          }
        };

        println!(
          "{hide_cursor}{pos_start}{empty_scene}{pos_last}\
          CTRL + Q to exit - FPS: {green}{fps}\
          {ui}{draws}",
          hide_cursor = console::seq::CURSOR_HIDE,
          pos_start = console::seq::CURSOR_START,
          ui = ui.unwrap_or_default(),
          fps = self.fps_capper.fps,
        );
        self.fps_capper.cap(timer.elapsed().as_millis().try_into().unwrap());
      }
    } else {
      console::fg(console::FG::BrightRed);
      println!("Could not get console size");
      console::reset();
    }
  }

  pub fn set_fps(&mut self, fps: u32) -> &mut Self {
    self.fps_capper.set(fps);
    self
  }
}

struct FPSCapper {
  pub fps: u32,
  fps_limit: u32,
  ms_per_frame: u32,
}

impl FPSCapper {
  pub fn new(fps_limit: u32) -> Self {
    Self {
      fps: fps_limit,
      fps_limit,
      ms_per_frame: if fps_limit > 0 {1000 / fps_limit} else {0}
    }
  }

  pub fn set(&mut self, fps_limit: u32) -> &mut Self {
    self.fps_limit = fps_limit;
    self.ms_per_frame = if fps_limit > 0 {1000 / fps_limit} else {0};
    self
  }

  pub fn cap(&mut self, last_frame_ms: u32) {
    if self.fps_limit > 0 && last_frame_ms < self.ms_per_frame {
      let ms_to_sleep = self.ms_per_frame - last_frame_ms;
      std::thread::sleep(Duration::from_millis(ms_to_sleep.try_into().unwrap()));
      self.fps = self.fps_limit;
    } else {
      self.fps = if last_frame_ms > 0 {1000 / last_frame_ms} else {1000};
    }
  }
}

pub struct Context {
  pub console_size: Size,
  pub timedelta: f64,
  pub event: Option<Event>,
}

impl Context {
  pub fn new() -> Self {
    Self { console_size: (0, 0).into(), timedelta: 0.0, event: None }
  }
}

pub struct Size {
  pub width: u16,
  pub height: u16,
}

impl From<(u16, u16)> for Size {
  fn from(size: (u16, u16)) -> Self {
    Self { width: size.0, height: size.1 }
  }
}

pub enum Group<'a, T> {
  Single(&'a T),
  Multi(&'a [T]),
}

pub trait EventHandler {
  fn update(&mut self, ctx: &Context) -> (Group<Pixel>, Option<String>);
}

pub mod event {
  pub fn to_key_event(event_option: Option<super::Event>) -> Option<super::KeyEvent> {
    if let Some(event) = event_option {
      if let super::Event::Key(key_event) = event {
        return Some(key_event);
      }
    }
    None
  }
}
