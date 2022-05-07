#![allow(unused)]
// use std::env::args;
// use std::path::PathBuf;
use clap::Parser;

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
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file"); // quit with message when error, 
    
    // 한줄씩 읽는다
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
