use rand::{Rng, distributions::uniform::SampleRange};
use std::{io::{Write, Stdout}, ops::Range, cmp::min};
use termion::{clear, color::{self, Rgb}, style, terminal_size, cursor};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

fn get_random_char() -> char {
  let mut rng = rand::thread_rng();
  let idx = rng.gen_range(0..CHARSET.len());
  CHARSET[idx] as char
}

fn get_random_color() -> color::Rgb {
  let mut rng = rand::thread_rng();
  let (r,g,b) = (rng.gen_range(0..=255), rng.gen_range(0..=255),rng.gen_range(0..=255));
  color::Rgb(r,g,b)
}

fn get_random_number(range: Range<u16>) -> u16 {
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}

#[derive(Debug, Clone)]
struct Datum {
  character: char,
  color: color::Rgb
}

impl Datum {
  fn set_black(&mut self) {
    self.color = color::Rgb(0,0,0);
  }

  fn set_lead_color(&mut self) {
    // 흰색으로 표기
    self.color = color::Rgb(230,230,230);
  }
  
  fn darken(&mut self) {
    let Rgb(r,g,b) = self.color;
    let darkness_factor = 0.5;
    // self.color = color::Rgb(r * darkness_factor, g * darkness_factor, b * darkness_factor);
  }
  
}

#[derive(Debug, Clone)]
pub struct DataString {
  data: Box<[Datum]>,
  visible_length: u16,
  pub x: u16,
  y_head: u16,
  matrix_width: u16,
  matrix_height: u16
}

impl DataString {
  pub fn new(width: u16, height: u16) -> DataString {
    DataString { 
      data: (0..height).map(|_| Datum {
      character: get_random_char(),
      color: get_random_color()
      }).collect(), 
      visible_length: get_random_number(8..20), 
      x: get_random_number(1..width),
      y_head: get_random_number(1..height-20),
      matrix_width: width,
      matrix_height: height
    }
  }
}

impl DataString {
  
  fn reset(&mut self) {
    self.data = (0..self.matrix_height).map(|_| Datum {
      character: get_random_char(),
      color: get_random_color()
      }).collect();
    self.visible_length = get_random_number(8..20);
    self.x = get_random_number(1..self.matrix_width);
    self.y_head = 0;
  }


  pub fn update(&mut self) {
    // 화면 밖임
    if let Some(res) = self.y_head.checked_sub(self.visible_length) {
      if res == self.matrix_height {
        self.reset();
      }
    };
    // 아래로 이동
    self.y_head += 1;
    self.y_head = min(self.y_head, self.matrix_height);
    if self.y_head == self.matrix_height {
      self.visible_length -= 1;
    }

  }

  // TODO: 겹치는 경우
  // TODO: interval다르게 draw
  pub fn draw<W: Write>(&self, stdout: &mut W) {

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
    
    for i in 1..window_min {
      write!(stdout, "{}{}{}", 
        cursor::Goto(self.x, i), 
        color::Black.fg_str(),
        ' ' 
      );
    }
  } 
}
