use clap::Parser;
use std::ops::Range;
use rand::Rng;
use termion::color;
use crate::libs::charset::{self, Charset};


#[derive(Parser)]
#[clap(author, version, about = "", long_about = None)]
struct Arguments {
  /// Set color of tail characters 
  ///
  /// OPTIONS:
  ///   white,
  ///   red,
  ///   blue,
  ///   green,
  ///   magenta,
  ///   cyan,
  ///   yellow,
  ///   random,
  ///   rainbow,
  ///   r,g,b
  #[clap(short, long, default_value_t = String::from("green"))]
  pub tail: String,

  /// Set color of a head character
  #[clap(short, long, default_value_t = String::from("white"))]
  pub head: String,

  /// Set charset of characters  
  /// 
  /// OPTIONS:
  ///   ascii,
  ///   katakana,
  ///    
  #[clap(short, long, default_value_t = String::from("ascii"))]
  pub charset: String,
}

pub fn parse_cli_arguments() -> Settings {
  let arguments: Arguments = Arguments::parse();
  // curryied color func
  let color = Box::new(move || {get_color_from_string(arguments.tail.as_str())});
  let charset = match arguments.charset.as_str() {
    "aascii" => Charset::AlphaNumSym,
    "katakana" => Charset::Katakana,
    _ => Charset::AlphaNumSym
  };
  
  Settings {
    color,
    charset
  }
}

pub struct Settings {
  // https://users.rust-lang.org/t/storing-a-function-taking-a-function-in-a-struct/14434
  pub color: Box<dyn Fn() -> color::Rgb>,
  pub charset: Charset
  
}



fn get_color_from_string(string: &str) -> color::Rgb {
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

fn get_random_color() -> color::Rgb {
  let mut rng = rand::thread_rng();
  let (r,g,b) = (rng.gen_range(0..=255), rng.gen_range(0..=255),rng.gen_range(0..=255));
  color::Rgb(r,g,b)
}

fn string_tuple_to_rgb(string_tuple: &str) -> color::Rgb {
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
