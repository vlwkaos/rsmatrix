use std::io::{Write, Stdout};

use termion::{clear, color, style, terminal_size};

#[derive(Debug, Clone)]
struct Cell {
  character: char,
  color: color::Rgb
}

#[derive(Debug)]
pub struct Matrix<W: Write> {
    grid: Box<[Cell]>,
    stdout: W,
    // stdin: R,
}

impl<W: Write> Matrix<W> {
  pub fn new(mut stdout: W, width: u16, height:u16) -> Matrix<W> {
    Matrix {
      grid: vec![Cell {
        color: color::Rgb(1,1,1),
        character: 'T'
      }; width as usize * height as usize].into_boxed_slice(),
      stdout: stdout
    }
  }
}

impl<W: Write> Matrix<W> {
  pub fn update(&self) {
  }

  pub fn draw(&self) {
  }
  
  fn clear(&mut self) {
    write!(self.stdout, "{}", clear::All).unwrap();
  }
}
