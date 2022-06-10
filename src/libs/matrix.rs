use rand::Rng;
use std::io::{Read, Stdin, Stdout, Write};
use termion::{
    clear,
    color::{self, Rgb},
    cursor,
    input::Keys,
    style, terminal_size,
};

use crate::arguments::Settings;

use super::{datastring::DataString, charset::CharWidth};
use super::{charset::Charset, drawable::Drawable};

// change queue를 만들어서 cursor로 움직여서...

pub struct Matrix<'a> {
    queue: Box<[DataString<'a>]>,
    // stdin: R,
    width: u16,
    height: u16,
}

impl Matrix<'_> {

    pub fn new(width: u16, height: u16, settings: &Settings) -> Matrix {
        let datastring_count = match settings.charset.get_width() {
          CharWidth::Half => width + 1,
          CharWidth::Full => width / u16::from(settings.charset.get_width()),
        };

        Matrix {
            queue: (1..datastring_count)
                .map(|i| DataString::new(i, width, height, settings))
                .collect(),
            width,
            height,
        }
    }
}

impl Drawable for Matrix<'_> {
    // reveal what was there
    // darken after a certain length
    fn update(&mut self, frame_count: u16) {
        for ds in self.queue.iter_mut() {
            ds.update(frame_count);
        }
    }

    fn draw<W: Write>(&self, stdout: &mut W) {
        for (i, ds) in self.queue.iter().enumerate() {
            // dbg
            // write!(stdout, "{}{}{}:{}", cursor::Goto(1,i as u16), color::White.fg_str() ,i, ds.x);
            ds.draw(stdout);
        }
    }
}
