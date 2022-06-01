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
      x: get_random_number(1..width),
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
    self.x = get_random_number(1..self.matrix_width);
    self.y_head = 0;
  }
  
  fn move_down(&mut self) {
    self.y_head += 1;
    self.y_head = min(self.y_head, self.matrix_height);
    if self.y_head == self.matrix_height {
      self.visible_length -= 1;
    }
  }
}
  
impl Drawable for DataString<'_> {

  fn update(&mut self, frame_count: u16) {
    // 화면 밖임
    if let Some(res) = self.y_head.checked_sub(self.visible_length) {
      if res == self.matrix_height {
        self.reset();
      }
    };
    // 아래로 이동
    if frame_count % self.update_frequency == 0 {
      self.move_down();
    }
  }

  fn draw<W: Write>(&self, stdout: &mut W) {

    let window_min = match self.y_head.checked_sub(self.visible_length) {
      Some(min) => min+1,
      None => 1
    };
    for i in window_min..self.y_head {
      write!(stdout, "{}{}{}", 
        cursor::Goto(self.x, i), 
        self.data[i as usize-1].color.fg_string(),
        self.data[i as usize-1].character
      );
    }
    
    // 마지막 두 셀을 삭제
    // TODO 코드 정리
    if window_min - 1 >= 1 {
      write!(stdout, "{}{}{}", 
        cursor::Goto(self.x, window_min-1), 
        color::Black.fg_str(),
        ' ' 
      );
    }

    write!(stdout, "{}{}{}", 
      cursor::Goto(self.x, window_min), 
      color::Black.fg_str(),
      ' ' 
    );
  } 
}
