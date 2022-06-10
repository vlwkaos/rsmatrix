#![allow(unused)]
mod libs;
mod arguments;

// use std::env::args;
// use std::path::PathBuf;
use termion::{clear, color, style, terminal_size, raw::IntoRawMode, cursor, input::TermRead, event::Key};
use core::time;
use std::{io::{self, Read, Write}, thread, time::{Instant, Duration}};
use libs::matrix::Matrix;
use libs::drawable::Drawable;

use arguments::parse_cli_arguments;

fn main() {
    // read cli arguments
    let settings = parse_cli_arguments();
    

    // lock stdout
    let stdout = io::stdout();
    let mut stdout = stdout.lock(); // stdout handle
    let mut stdout = stdout.into_raw_mode().unwrap(); // 정교한 제어를 위해 raw mode
    
    // lock stdin
    let mut stdin = termion::async_stdin();
    let mut it = stdin.keys();
    
    let terminal_size = termion::terminal_size().ok();
    let width = terminal_size.map(|(w,_)| w).unwrap_or(100);
    let height = terminal_size.map(|(_,h)| h).unwrap_or(100);

    // set buffer size
    let mut stdout = io::BufWriter::with_capacity(width as usize * height as usize * 2, stdout);
        
    // initialize matrix
    let mut matrix = Matrix::new(width, height, &settings);

    // start drawing
    let frames_per_second: u16 = settings.frames;
    let mili_per_frame: u16 = 1000/frames_per_second;
    let mut frame_count: u16 = 0;
    write!(stdout, "{}{}", cursor::Hide, clear::All);
    loop {
        let now = Instant::now();
        // user input while running
        let b = it.next();
        if let Some(event) = b {
            match event {
                Ok(Key::Char('q')) => break,          
                _ => {}
            }     
        }
        
        // update and draw
        matrix.update(frame_count);
        matrix.draw(&mut stdout);
        
        let elasped_mili = now.elapsed().as_millis();

        // increment frame_count and reset if overflow
        frame_count += 1;
        if frame_count > frames_per_second * 10 {
            frame_count = 0;
            stdout.flush();
        }

        // wait
        match mili_per_frame.checked_sub(elasped_mili as u16) {
            Some(dif) => {
                thread::sleep(time::Duration::from_millis(dif as u64));
            },
            None => {}
        };
    }
    
    // stderr.flush().unwrap();
}
