use crate::arguments::{Settings, Trilean};
use rand::{distributions::uniform::SampleRange, Rng};
use std::{
    cmp::min,
    io::{Stdout, Write},
    ops::Range,
};
use termion::{
    clear,
    color::{self, Rgb},
    cursor, style, terminal_size,
};

use super::utils::*;
use super::{charset::Charset, drawable::Drawable};

#[derive(Clone)]
struct Datum {
    character: char,
    color: color::Rgb,
    head: color::Rgb,
    bold: Trilean,
}

impl Datum {
    
    fn get_bold(&self, is_head: bool) -> String {
        let bold;
        if is_head {
            bold = self.bold.get_optimistic_bool();
        } else {
            bold = self.bold.get_bool();
        }
        match bold {
            true => style::Bold.to_string(),
            false => style::NoBold.to_string(),
        }
    }
    
    fn get_color(&self, is_head: bool) -> String {
        if is_head {
            self.head.fg_string()
        } else {
            self.color.fg_string()
        }
    }

    fn draw<W: Write>(&self, stdout: &mut W, is_head: bool) {

        write!(
            stdout,
            "{}{}{}{}",
            self.get_color(is_head),
            self.get_bold(is_head),
            self.character,
            style::Reset.to_string()
        );
    }
    
}

#[derive(Clone)]
pub struct DataString<'a> {
    data: Box<[Datum]>,
    visible_length: u16,
    x: u16,
    y_head: u16,
    matrix_width: u16,
    matrix_height: u16,
    update_frequency: u16,
    settings: &'a Settings,
}

impl DataString<'_> {
    pub fn new(x: u16, width: u16, height: u16, settings: &Settings) -> DataString {
        DataString {
            data: (0..height)
                .map(|_| Datum {
                    character: settings.charset.get_random_char(),
                    color: (settings.get_tail_color)(),
                    head: (settings.get_head_color)(),
                    bold: settings.bold,
                })
                .collect(),
            visible_length: get_random_number(height / 4..height / 2),
            x,
            y_head: get_random_number(1..height),
            matrix_width: width,
            matrix_height: height,
            update_frequency: get_random_number(4..8),
            settings,
        }
    }
}

impl DataString<'_> {
    fn reset(&mut self) {
        self.data = (0..self.matrix_height)
            .map(|_| Datum {
                character: self.settings.charset.get_random_char(),
                color: (self.settings.get_tail_color)(),
                head: (self.settings.get_head_color)(),
                bold: self.settings.bold,
            })
            .collect();
        self.visible_length = get_random_number(self.matrix_height / 4..self.matrix_height / 2);
        self.y_head = 0;
    }

    fn move_down(&mut self) {
        self.y_head += 1;
    }

    fn get_y_tail(&self) -> Option<u16> {
        match self.y_head.checked_sub(self.visible_length) {
            Some(min) => Some(min + 1),
            None => None,
        }
    }

    fn draw_head<W: Write>(&self, stdout: &mut W) {
        if 1 <= self.y_head && self.y_head <= self.matrix_height + 1 {
            // head
            if self.y_head <= self.matrix_height {
                stdout.write(cursor::Goto( self.x * u16::from(self.settings.charset.get_width()), self.y_head).to_string().as_bytes());
                self.data[(self.y_head - 1) as usize].draw(stdout, true);
            }

            // tail
            let neck = self.y_head - 1;
            if 1 <= neck {
                stdout.write( cursor::Goto(self.x * u16::from(self.settings.charset.get_width()), neck).to_string().as_bytes());
                self.data[(neck - 1) as usize].draw(stdout, false);
            }
        }
    }

    fn remove_tail<W: Write>(&self, stdout: &mut W) {
        if let Some(y_tail) = self.get_y_tail() {
            write!(
                stdout,
                "{}{}{}",
                cursor::Goto(
                    self.x * u16::from(self.settings.charset.get_width()),
                    y_tail - 1
                ),
                color::Black.fg_str(),
                ' '
            );
        }
    }
}

impl Drawable for DataString<'_> {
    fn update(&mut self, frame_count: u16) {
        // 화면 밖임
        if let Some(y) = self.get_y_tail() {
            if y > self.matrix_height {
                self.reset();
            }
        };
        // 아래로 이동
        if frame_count % self.update_frequency == 0 {
            self.move_down();
        }
    }

    fn draw<W: Write>(&self, stdout: &mut W) {
        // draw string
        self.draw_head(stdout);
        // erase tail
        self.remove_tail(stdout);
    }
}
