use console;
mod drawable;

use std::{rc::Rc, cell::RefCell, time::Duration};
pub use drawable::Drawable;
use crossterm::event::{poll, read};
pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Renderer {
  fps: u32,
  handler: Rc<RefCell<dyn EventHandler>>,
}

impl Renderer {
  pub fn new(handler: Rc<RefCell<dyn EventHandler>>) -> Self {
    Self { fps: 0, handler }
  }

  pub fn run(&mut self) {
    if let Some((w, h)) = console::size() {
      console::clear();
      let exit_event = Event::Key(KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::CONTROL,
      });
      let empty_scene = " ".repeat(w.into()).repeat(h.into());
      let pos_last = console::seq::goto(1, h);
      let mut fps = 0;

      loop {
        let timer = std::time::Instant::now();
        let mut handler_borrow = self.handler.borrow_mut();
        let drawables: &Vec<Drawable>;
        if let Ok(true) = poll(Duration::from_millis(0)) {
          let event = read().unwrap();
          if event == exit_event {
            console::reset();
            console::show_cursor();
            console::goto(1, h);
            println!("\nExiting...");
            break
          }
          drawables = handler_borrow.update(Some(event));
        } else {
          drawables = handler_borrow.update(None);
        }

        let mut drawable_draws: Vec<String> = Vec::with_capacity(drawables.len());
        for drawable in drawables.iter() {
          drawable_draws.push(drawable.to_string());
        }

        print!(
          "{hide_cursor}{pos_start}{empty_scene}{pos_last}\
          CTRL + Q to exit - FPS: {green}{fps}\
          {draws}{show_cursor}",
          hide_cursor = console::seq::CURSOR_HIDE,
          pos_start = console::seq::CURSOR_START,
          draws = drawable_draws.join(""),
          show_cursor = console::seq::CURSOR_SHOW,
          green = console::seq::fg_rgb(150, 255, 120)
        );
        fps = Renderer::sync_fps(self.fps, timer.elapsed().as_millis().try_into().unwrap());
      }
    } else {
      console::fg(console::FG::BrightRed);
      println!("Could not get console size");
      console::reset();
    }
  }

  pub fn set_fps(&mut self, fps: u32) -> &mut Self {
    self.fps = fps;
    self
  }

  fn sync_fps(limit: u32, elapsed_ms: u32) -> i64 {
    if limit > 0 && limit > elapsed_ms {
      let fps = (limit - elapsed_ms) as i64;
      let ms_to_sleep: i64 = (1000 as i64) / fps;
      if ms_to_sleep > 0 {
        std::thread::sleep(Duration::from_millis(ms_to_sleep.try_into().unwrap()));
        fps
      } else { 0 }
    } else { 0 }
  }
}

pub trait EventHandler {
  fn update(&mut self, event: Option<Event>) -> &Vec<Drawable> ;
}

pub fn is_key_pressed(event: Event, code: KeyCode) -> bool {
  event == Event::Key(code.into())
}
