#![allow(unused)]
mod matrix;


// use std::env::args;
// use std::path::PathBuf;
use clap::Parser;
use termion::{clear, color, style, terminal_size, raw::IntoRawMode};
use core::time;
use std::{io::{self, Read, Write}, thread};
use matrix::Matrix;

const FRAME_DURATION: u64 = 60/1000; // 60 frame per sec

#[derive(Parser)]
struct Args {
    pattern: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    // 성능을 위해 stdios를 초기에 lock시킨다.
    let stdout = io::stdout();
    let mut stdout = stdout.lock(); // stdout handle
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    
    let terminal_size = termion::terminal_size().ok();
    let width = terminal_size.map(|(w,_)| w).unwrap_or(100);
    let height = terminal_size.map(|(_,h)| h).unwrap_or(100);
    
    // We go to raw mode to make the control over the terminal more fine-grained.
    let stdout = stdout.into_raw_mode().unwrap();
    let mut matrix = Matrix::new(stdout, width, height);
   
    // init(stdout, stdin, width.unwrap_or(100), height.unwrap_or(100));
    let mut is_running = true;
    
    // while is_running {
        matrix.draw();
        matrix.update();

        thread::sleep(time::Duration::from_millis(FRAME_DURATION));
        matrix.draw();
        matrix.update();

        thread::sleep(time::Duration::from_millis(FRAME_DURATION));
    // }
    
    // stderr.flush().unwrap();
}
