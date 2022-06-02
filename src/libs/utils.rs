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
    string_tuple => string_tuple_to_rgb(string_tuple) 
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