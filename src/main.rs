#![allow(unused)]
mod matrix;


// use std::env::args;
// use std::path::PathBuf;
use clap::Parser;
use termion::{clear, color, style, terminal_size};
use std::io::{self, Read, Write};
use matrix::Matrix;

#[derive(Parser)]
struct Args {
    pattern: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    // 성능을 위해 stdios를 초기에 lock시킨다.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    
    let terminal_size = termion::terminal_size().ok();
    let width = terminal_size.map(|(w,_)| w).unwrap_or(100);
    let height = terminal_size.map(|(_,h)| h).unwrap_or(100);
    
    let mut matrix = Matrix::new(stdout, width, height);
   
    // init(stdout, stdin, width.unwrap_or(100), height.unwrap_or(100));
    let mut is_running = true;

    // while is_running {
    //     draw(stdout)?;
    //     stdout.flush()?;
    //     update();
    // }
    
    // stdout.flush().unwrap();
    // stderr.flush().unwrap();
}
