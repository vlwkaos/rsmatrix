use std::ops::Range;

use rand::Rng;
use termion::color;

pub fn get_random_number(range: Range<u16>) -> u16 {
  let mut rng = rand::thread_rng();
  rng.gen_range(range)
}

pub fn string_tuple_to_rgb(string_tuple: &str) -> color::Rgb {
  let mut rgb_vec = Vec::new();
  for i in string_tuple.split(',') {
    rgb_vec.push(
      i.parse::<u8>()
      .expect("Please enter correct color: r,g,b or green.")
    );
  }
  
  let r = rgb_vec[0];
  let g = rgb_vec[1];
  let b = rgb_vec[2];
  color::Rgb(r,g,b)
}