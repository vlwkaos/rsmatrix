#![allow(unused)]
mod libs;

// use std::env::args;
// use std::path::PathBuf;
use clap::Parser;
use termion::{clear, color, style, terminal_size, raw::IntoRawMode, cursor, input::TermRead, event::Key};
use core::time;
use std::{io::{self, Read, Write}, thread};
use libs::matrix::Matrix;

const FRAME_DURATION: u64 = 100; // 

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

    write!(stdout, "{}{}", cursor::Hide, clear::All);
    loop {
        let b = it.next();
        if let Some(event) = b {
            match event {
                Ok(Key::Char('q')) => break,          
                _ => {}
            }     
        }
        matrix.update();
        matrix.draw(&mut stdout);
        // stdout.flush();
        thread::sleep(time::Duration::from_millis(FRAME_DURATION));
    }
    
    // stderr.flush().unwrap();
}
