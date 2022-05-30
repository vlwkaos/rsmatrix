use std::ops::Range;

use rand::Rng;
use termion::color;

pub fn get_random_color() -> color::Rgb {
  let mut rng = rand::thread_rng();
  let (r,g,b) = (rng.gen_range(0..=255), rng.gen_range(0..=255),rng.gen_range(0..=255));
  color::Rgb(r,g,b)
}

pub fn get_random_number(range: Range<u16>) -> u16 {
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}