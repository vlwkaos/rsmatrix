#![allow(unused)]
mod libs;
mod arguments;

// use std::env::args;
// use std::path::PathBuf;
use termion::{clear, color, style, terminal_size, raw::IntoRawMode, cursor, input::TermRead, event::Key};
use core::time;
use std::{io::{self, Read, Write}, thread};
use libs::matrix::Matrix;
use libs::drawable::Drawable;

use arguments::parse_cli_arguments;


const FRAME: u16 = 60;
const FRAME_DURATION: u16 = 1000/FRAME; // 60fps

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
    
    // initialize matrix
    let terminal_size = termion::terminal_size().ok();
    let width = terminal_size.map(|(w,_)| w).unwrap_or(100);
    let height = terminal_size.map(|(_,h)| h).unwrap_or(100);
    let mut matrix = Matrix::new(width, height, settings.color.as_str());

    // start drawing
    let mut frame_count: u16 = 0;
    write!(stdout, "{}{}", cursor::Hide, clear::All);
    loop {
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
        // stdout.flush();
        // increment frame_count and reset if overflow
        frame_count += 1;
        if frame_count > FRAME * 10 {
            frame_count = 0;
        }
        
        // update interval
        thread::sleep(time::Duration::from_millis(FRAME_DURATION as u64));
    }
    
    // stderr.flush().unwrap();
}
