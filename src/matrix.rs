use std::io::{Write, Stdout};

use termion::{clear, color, style, terminal_size};

// TODO: impl Display for Cell / Matrix

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
    width: u16,
    height: u16,
}

impl<W: Write> Matrix<W> {
  pub fn new(mut stdout: W, width: u16, height:u16) -> Matrix<W> {
    Matrix {
      grid: vec![Cell {
        color: color::Rgb(1,1,1),
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
  }

  pub fn draw(&mut self) {
    // Q: How to concat string with fold
    // https://stackoverflow.com/questions/68854895/reduce-vs-fold-for-string-concatenation-in-rust
    let result = self.grid.iter()
      .map(|c| c.character.to_string())
      .fold("".to_string(), |cur, next| cur + &next);
    // Q: How to print lusing stdout
    // 1. impl Display trait
    // 2. direct
    write!(self.stdout, "{}", result);
  }
  
  pub fn clear(&mut self) {
    write!(self.stdout, "{}", clear::All).unwrap();
  }
}
