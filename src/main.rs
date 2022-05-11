#![allow(unused)]
// use std::env::args;
// use std::path::PathBuf;
use clap::Parser;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

// #[clap(short = 'o', long = "output")]
// https://docs.rs/clap/

fn main() {
    // ## 1.2 Parsing command line arguments
    // let pattern = args().nth(1).expect("no pattern given"); 
    // let path = args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern,
    //     path: PathBuf::from(path),
    // };
    let args = Cli::parse();
    // 실패해도 clap이 자동으로 help 메세지도 만들어줌
    // parse 메소드는 main에서만 사용할 것
    

    // ## 1.3 First Implementation
    // 파일을 읽어보자
    // TODO: improve with Nice error reporting
    let f = File::open(&args.path);
    let f = match f {
        Ok(file) => BufReader::new(file),
        Err(err) => {panic!("No such file found.");}
    };
    
    // 한줄씩 읽는다
    for line in f.lines() {
        // pattern을 포함하는 경우만 출력
        let l = line.unwrap();
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }
}
