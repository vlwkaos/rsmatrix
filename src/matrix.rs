use rand::Rng;
use std::{io::{Write, Stdout}, cell::Cell};

use termion::{clear, color::{self, Rgb}, style, terminal_size, cursor};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

// TODO: impl Display for Cell / Matrix

#[derive(Debug, Clone)]
struct Block {
  character: char,
  color: color::Rgb
}

impl Block {
  // 다음 글자를 구한다 mutable fn
  fn random_char(&mut self) {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..CHARSET.len());
    self.character = CHARSET[idx] as char
  }

  fn random_color(&mut self) {
    // 다음 색상을 구한다
    let mut rng = rand::thread_rng();
    let (r,g,b) = (rng.gen_range(0..=255), rng.gen_range(0..=255),rng.gen_range(0..=255));
    self.color = color::Rgb(r,g,b);
  }
}

// change queue를 만들어서 cursor로 움직여서...

#[derive(Debug)]
pub struct Matrix<W: Write> {
    grid: Box<[Block]>,
    stdout: W,
    // stdin: R,
    width: u16,
    height: u16,
}

impl<W: Write> Matrix<W> {
  pub fn new(mut stdout: W, width: u16, height:u16) -> Matrix<W> {
    Matrix {
      grid: vec![Block {
        color: color::Rgb(200,255,200),
        character: 'T' 
      }; width as usize * height as usize].into_boxed_slice(),
      stdout,
      width,
      height
    }
  }
}

impl<W: Write> Matrix<W> {
  
  pub fn randomize(&mut self) {
    for (b) in self.grid.iter_mut() {
      b.random_char();
      b.random_color();
    }
  }
  
  pub fn update(&mut self) {
    for i in (0..self.grid.len()).rev() {
      // set to character above, move every line down by 1
      // vector idx is only usize, so must be checked
      let prev_char;
      let prev_color;
      match i.checked_sub(self.width as usize) {
        Some(idx) => {
          prev_char = self.grid[idx].character;
          prev_color = self.grid[idx].color;
        }, 
        None => {
          prev_char = ' ';
          prev_color = Rgb(0, 0, 0);
        }
      };
      
      let mut block = &mut self.grid[i];
      block.character = prev_char;
      block.color = prev_color;
    }
  }

  pub fn draw(&mut self) {
    write!(self.stdout, "{}{}", cursor::Hide, cursor::Goto(1,1));
    // Q: How to concat string with fold
    // https://stackoverflow.com/questions/68854895/reduce-vs-fold-for-string-concatenation-in-rust
    let result = self.grid.iter()
      .fold("".to_string(), |cur, next| cur + &next.color.fg_string() + &next.character.to_string());
    // Q: How to print lusing stdout
    // 1. impl Display trait
    // 2. direct
    write!(self.stdout, "{}{}", result, clear::AfterCursor);
    self.stdout.flush();
  }
  
}
