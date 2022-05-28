#![allow(unused)]
mod libs;

// use std::env::args;
// use std::path::PathBuf;
use clap::Parser;
use termion::{clear, color, style, terminal_size, raw::IntoRawMode, cursor, input::TermRead, event::Key};
use core::time;
use std::{io::{self, Read, Write}, thread};
use libs::matrix::Matrix;
use libs::drawable::Drawable;

const FRAME: u16 = 60;
const FRAME_DURATION: u16 = 1000/FRAME; // 60fps

#[derive(Parser)]
struct Args {
    pattern: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    // 성능을 위해 stdios를 초기에 lock시킨다.
    let stdout = io::stdout();
    let mut stdout = stdout.lock(); // stdout handle
    let mut stdout = stdout.into_raw_mode().unwrap(); // 정교한 제어를 위해 raw mode
    
    // in
    let mut stdin = termion::async_stdin();
    let mut it = stdin.keys();

    // let stderr = io::stderr();
    // let mut stderr = stderr.lock();
    
    let terminal_size = termion::terminal_size().ok();
    let width = terminal_size.map(|(w,_)| w).unwrap_or(100);
    let height = terminal_size.map(|(_,h)| h).unwrap_or(100);
    let mut matrix = Matrix::new(width, height);

    let mut frame_count: u16 = 0;
    write!(stdout, "{}{}", cursor::Hide, clear::All);
    loop {
        let b = it.next();
        if let Some(event) = b {
            match event {
                Ok(Key::Char('q')) => break,          
                _ => {}
            }     
        }
        matrix.update(frame_count);
        matrix.draw(&mut stdout);
        // stdout.flush();
        // increment frame_count and reset if overflow
        frame_count += 1;
        if frame_count > FRAME * 10 {
            frame_count = 0;
        }
        thread::sleep(time::Duration::from_millis(FRAME_DURATION as u64));
    }
    
    // stderr.flush().unwrap();
}
