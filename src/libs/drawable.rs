use std::io::Write;

pub trait Drawable {
  fn update(&mut self, frame_count: u16); 
  fn draw<W: Write>(&self, stdout: &mut W);
}