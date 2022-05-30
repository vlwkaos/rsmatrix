use rand::Rng;
use std::{io::{Write, Stdout, Read, Stdin}};
use termion::{clear, color::{self, Rgb}, style, terminal_size, cursor, input::Keys};

use super::drawable::Drawable;
use super::datastring::DataString;

// change queue를 만들어서 cursor로 움직여서...

#[derive(Debug)]
pub struct Matrix {
  queue: Box<[DataString]>,
  // stdin: R,
  width: u16,
  height: u16,
}

impl Matrix {
  pub fn new(width: u16, height:u16) -> Matrix {
    
    Matrix {
      queue: (0..width/2).map(|_| DataString::new(
        width, 
        height,
        super::charset::Charset::Katakana
      )).collect(),
      width,
      height
    }
  }
}

impl Drawable for Matrix {
  
  // reveal what was there
  // darken after a certain length
  fn update(&mut self, frame_count: u16) {
    for ds in self.queue.iter_mut() {
      ds.update(frame_count);
    }
  }

  fn draw<W: Write>(&self, stdout: &mut W) {
    for (i,ds) in self.queue.iter().enumerate() {
      // dbg
      // write!(stdout, "{}{}{}:{}", cursor::Goto(1,i as u16), color::White.fg_str() ,i, ds.x); 
      ds.draw(stdout);
    }
  }
  
}
