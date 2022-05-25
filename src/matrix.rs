use rand::Rng;
use std::{io::{Write, Stdout}, cell::Cell};

use termion::{clear, color, style, terminal_size};

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
  fn next_char(&mut self) {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..CHARSET.len());
    self.character = CHARSET[idx] as char
  }

  fn next_color(&mut self) {
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
    write!(stdout, "{}", clear::All).unwrap();

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
  pub fn update(&mut self) {
    for (c) in self.grid.iter_mut() {
      c.next_char();
      c.next_color();
    }
  }

  pub fn draw(&mut self) {
    // Q: How to concat string with fold
    // https://stackoverflow.com/questions/68854895/reduce-vs-fold-for-string-concatenation-in-rust
    let result = self.grid.iter()
      .fold("".to_string(), |cur, next| cur + &next.color.fg_string() + &next.character.to_string());
    // Q: How to print lusing stdout
    // 1. impl Display trait
    // 2. direct
    write!(self.stdout, "{}", result);
    self.stdout.flush();
  }
  
}
