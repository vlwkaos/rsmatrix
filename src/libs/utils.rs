use std::ops::Range;

use rand::Rng;
use termion::color;

pub fn get_color_from_string(string: &str) -> color::Rgb {
  match string {
    "red" => color::Rgb(255,0,0),
    "green" => color::Rgb(0,255,0),
    "blue" => color::Rgb(0,0,255),
    "white" => color::Rgb(255,255,255),
    "yellow" => color::Rgb(255,255,0),
    "magenta" => color::Rgb(255,0,255),
    "cyan" => color::Rgb(0,255,255),
    "random" => get_random_color(),
    _ => color::Rgb(0, 255, 0)
  }
}


pub fn get_random_color() -> color::Rgb {
  let mut rng = rand::thread_rng();
  let (r,g,b) = (rng.gen_range(0..=255), rng.gen_range(0..=255),rng.gen_range(0..=255));
  color::Rgb(r,g,b)
}

pub fn get_random_number(range: Range<u16>) -> u16 {
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}