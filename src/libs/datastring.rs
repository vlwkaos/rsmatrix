use rand::{Rng, distributions::uniform::SampleRange};
use std::{io::{Write, Stdout}, ops::Range, cmp::min};
use termion::{clear, color::{self, Rgb}, style, terminal_size, cursor};
use super::{drawable::Drawable, charset::Charset};
use super::utils::*;

#[derive(Debug, Clone)]
struct Datum {
  character: char,
  color: color::Rgb
}

#[derive(Debug, Clone)]
pub struct DataString<'a> {
  data: Box<[Datum]>,
  visible_length: u16,
  x: u16,
  y_head: u16,
  matrix_width: u16,
  matrix_height: u16,
  update_frequency: u16,
  charset: Charset,
  color: &'a str
}

impl DataString<'_> {
  pub fn new(width: u16, height: u16, charset: Charset, color: &str) -> DataString {

    DataString { 
      data: (0..height).map(|_| Datum {
      character: charset.get_random_char(),
      color: get_color_from_string(color)
      }).collect(), 
      visible_length: get_random_number(8..20), 
      x: get_random_number(1..(width / charset.get_width())),
      y_head: get_random_number(1..height-20),
      matrix_width: width,
      matrix_height: height,
      update_frequency: get_random_number(1..10),
      charset,
      color
    }
  }
}

impl DataString<'_> {
  fn reset(&mut self) {
    self.data = (0..self.matrix_height).map(|_| Datum {
      character: self.charset.get_random_char(),
      color: get_color_from_string(self.color)
      }).collect();
    self.visible_length = get_random_number(8..20);
    self.x = get_random_number(1..(self.matrix_width / self.charset.get_width()));
    self.y_head = 0;
  }
  
  fn move_down(&mut self) {
    self.y_head += 1;
  }
  
  fn get_y_tail(&self) -> Option<u16> {
    match self.y_head.checked_sub(self.visible_length) {
      Some(min) => {
          Some(min+1)
      },
      None => None
    }
  }
  
}
  
impl Drawable for DataString<'_> {

  fn update(&mut self, frame_count: u16) {
    // 화면 밖임
    if let Some(y) = self.get_y_tail() {
      if y > self.matrix_height {
        self.reset();
      }
    };
    // 아래로 이동
    if frame_count % self.update_frequency == 0 {
      self.move_down();
    }
  }

  fn draw<W: Write>(&self, stdout: &mut W) {

    // to end 
    if let Some(y_tail) = self.get_y_tail() {
      for i in y_tail..=self.y_head {
        if i <= self.matrix_height {
          write!(stdout, "{}{}{}", 
          cursor::Goto(self.x * self.charset.get_width(), i), 
            self.data[(i-1) as usize].color.fg_string(),
            self.data[(i-1) as usize].character
          );
        }
      }
      // erase tail
      write!(stdout, "{}{}{}", 
        cursor::Goto(self.x * self.charset.get_width(), y_tail-1), 
        color::Black.fg_str(),
        ' ' 
      );
    } else {
      // start
      for i in 1..=self.y_head {
        write!(stdout, "{}{}{}", 
          cursor::Goto(self.x * self.charset.get_width(), i), 
          self.data[(i-1) as usize].color.fg_string(),
          self.data[(i-1) as usize].character
        );
      }
      
    };

  } 
}
