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
    bold: bool,
}

impl Datum {
    fn get_bold(&self) -> String {
        match self.bold {
            true => style::Bold.to_string(),
            false => style::NoBold.to_string(),
        }
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
                    bold: settings.bold.get_bool(),
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
                bold: self.settings.bold.get_bool(),
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
        let bold = match self.settings.bold.get_optimistic_bool() {
            true => style::Bold.to_string(),
            false => style::NoBold.to_string(),
        };

        write!(
            stdout,
            "{}{}{}{}{}",
            cursor::Goto(self.x * u16::from(self.settings.charset.get_width()), self.y_head),
            (self.settings.get_head_color)().fg_string(),
            bold,
            self.data[(self.y_head - 1) as usize].character,
            style::Reset.to_string()
        );
    }

    fn draw_tail<W: Write>(&self, stdout: &mut W) {
        let neck = self.y_head - 1;
        write!(
            stdout,
            "{}{}{}{}{}",
            cursor::Goto(self.x * u16::from(self.settings.charset.get_width()), neck),
            self.data[(neck - 1) as usize].color.fg_string(),
            self.data[(neck - 1) as usize].get_bold(),
            self.data[(neck - 1) as usize].character,
            style::Reset.to_string()
        );
    }
    
    fn remove_tail<W: Write>(&self, stdout: &mut W) {
        if let Some(y_tail) = self.get_y_tail() {
            write!(
                stdout,
                "{}{}{}",
                cursor::Goto(self.x * u16::from(self.settings.charset.get_width()), y_tail - 1),
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
        if 1 <= self.y_head && self.y_head <= self.matrix_height + 1 {
            // head
            if self.y_head <= self.matrix_height {
                self.draw_head(stdout);
            }

            // tail
            let neck = self.y_head - 1;
            if 1 <= neck {
                self.draw_tail(stdout);
            }
        }

        // erase tail
        self.remove_tail(stdout);
    }
}
