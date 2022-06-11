use crate::libs::{
    charset::{self, Charset},
    utils::get_random_number,
};
use std::{cell::RefCell, cmp::Ord};
use clap::Parser;
use rand::Rng;
use std::ops::Range;
use termion::color;

const RAINBOW: [color::Rgb; 7] = [
    color::Rgb(148, 0, 211),
    color::Rgb(75, 0, 130),
    color::Rgb(0, 0, 255),
    color::Rgb(0, 255, 0),
    color::Rgb(255, 255, 0),
    color::Rgb(255, 127, 0),
    color::Rgb(255, 0, 0),
];
thread_local! {
    static GLOBAL_ITER_IDX: RefCell<usize> = RefCell::new(0);
}

static mut RAINBOW_MANUAL_ITER: u8 = 0;

#[derive(Clone, Copy)]
pub enum Trilean {
    True,
    False,
    Unknown,
}

impl Trilean {
    pub fn get_bool(&self) -> bool {
        match self {
            Trilean::True => true,
            Trilean::False => false,
            Trilean::Unknown => {
                let b = get_random_number(0..2);
                if b == 0 {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn get_optimistic_bool(&self) -> bool {
        match self {
            Trilean::True => true,
            Trilean::False => false,
            Trilean::Unknown => true,
        }
    }
}

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
    #[clap(short, long, default_value_t = String::from("white"))]
    pub head: String,

    /// Set charset of characters displayed
    ///
    /// OPTIONS:
    ///   ascii,
    ///   katakana,
    ///    
    #[clap(short, long, default_value_t = String::from("ascii"))]
    pub charset: String,

    /// Set characters bold (random still sets head to bold)
    ///
    /// OPTIONS:
    ///   true,
    ///   false,
    ///   random,
    #[clap(short, long, default_value_t = String::from("random"))]
    pub bold: String,

    /// Set update frequency (the higher, the faster)
    #[clap(short, long, default_value_t = 120)]
    pub frames: u16,

    /// Set brightness effect for tail
    /// 
    /// OPTIONS:
    ///     none,
    ///     random,
    ///     gradient
    #[clap(short='l', long, default_value_t = String::from("random"))]
    pub brightness: String,
}

pub fn parse_cli_arguments() -> Settings {

    let arguments: Arguments = Arguments::parse();
    // curryied color func
    let get_tail_color = Box::new(move || get_color_from_string(arguments.tail.as_str(), arguments.brightness.as_str()));
    let get_head_color = Box::new(move || get_color_from_string(arguments.head.as_str(), "none"));

    let charset = match arguments.charset.as_str() {
        "aascii" => Charset::AlphaNumSym,
        "katakana" => Charset::Katakana,
        "emoji" => Charset::Emoji,
        _ => Charset::AlphaNumSym,
    };

    let bold = match arguments.bold.as_str() {
        "true" => Trilean::True,
        "false" => Trilean::False,
        "random" => Trilean::Unknown,
        _ => Trilean::Unknown,
    };

    Settings {
        get_tail_color,
        get_head_color,
        charset,
        bold,
        frames: arguments.frames,
    }
}

pub struct Settings {
    // https://users.rust-lang.org/t/storing-a-function-taking-a-function-in-a-struct/14434
    pub get_tail_color: Box<dyn Fn() -> color::Rgb>,
    pub get_head_color: Box<dyn Fn() -> color::Rgb>,
    pub charset: Charset,
    pub bold: Trilean,
    pub frames: u16,
}

fn get_color_from_string(color: &str, brightness: &str) -> color::Rgb {
    let ret = match color {
        "red" => color::Rgb(255, 0, 0),
        "green" => color::Rgb(0, 255, 0),
        "blue" => color::Rgb(0, 0, 255),
        "white" => color::Rgb(255, 255, 255),
        "yellow" => color::Rgb(255, 255, 0),
        "magenta" => color::Rgb(255, 0, 255),
        "cyan" => color::Rgb(0, 255, 255),
        "random" => get_random_color(),
        "rainbow" => get_next_rainbow_color(),
        string_tuple => string_tuple_to_rgb(string_tuple),
    };
    apply_brightness(ret, brightness)
}

fn get_lower_brightness(color: color::Rgb, factor: u8) -> color::Rgb {
    let color::Rgb(r,g,b) = color;
    let factor = factor + 1;
    let r = (r / factor).clamp(0, 255) as u8;
    let g = (g / factor).clamp(0, 255) as u8;
    let b = (b / factor).clamp(0, 255) as u8;
    color::Rgb(r,g,b)
}

fn apply_brightness(color: color::Rgb, brightness: &str) -> color::Rgb {
    match brightness {
        "none" => color,
        "random" => {
            get_lower_brightness(color, get_random_number(0..5) as u8)
        },
        "gradient" => {
            GLOBAL_ITER_IDX.with(|idx| {
                let i = *idx.borrow();
                let factor = if i > 10 { 20 - i } else {i};
                let ret = get_lower_brightness(color, factor as u8);
                *idx.borrow_mut() = (i+1)%20;
                ret 
            })
        },
        _ => color,
    }
}

fn get_next_rainbow_color() -> color::Rgb {
    GLOBAL_ITER_IDX.with(|idx| {
        let ret = RAINBOW[*idx.borrow() % 7];
        *idx.borrow_mut() += 1;
        ret
    })
}

fn get_random_color() -> color::Rgb {
    let mut rng = rand::thread_rng();
    let (r, g, b) = (
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
    );
    color::Rgb(r, g, b)
}

fn string_tuple_to_rgb(string_tuple: &str) -> color::Rgb {
    let mut rgb_vec = Vec::new();
    for i in string_tuple.split(',') {
        rgb_vec.push(
            i.parse::<u8>()
                .expect("Incorrect color format entered. Try r,g,b or green"),
        );
    }

    let r = rgb_vec[0];
    let g = rgb_vec[1];
    let b = rgb_vec[2];
    color::Rgb(r, g, b)
}
