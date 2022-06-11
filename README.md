# rsmatrix

<img width="900" alt="image" src="https://user-images.githubusercontent.com/44766242/173063572-8cc51cec-1c07-4d3f-add9-b1ccda24fd13.png">
<img width="900" alt="image" src="https://user-images.githubusercontent.com/44766242/173187513-ad74d355-84c1-4005-ac32-1500286fce5b.png">

![Screen Recording 2022-06-10 at 12 56 09 PM](https://user-images.githubusercontent.com/44766242/172987437-c2d6330a-4642-46c6-871f-1d7f79084eca.gif)

🚧 Work in Progress

My attempt at building [cmatrix](https://github.com/abishekvashok/cmatrix) clone using rust

```sh
USAGE:
    rsmatrix [OPTIONS]

OPTIONS:
    -b, --bold <BOLD>
            Set characters bold (random still sets head to bold)

            OPTIONS: true, false, random,

            [default: random]

    -c, --charset <CHARSET>
            Set charset of characters displayed

            OPTIONS: ascii, katakana,

            [default: ascii]

    -f, --frames <FRAMES>
            Set update frequency (the higher, the faster)

            [default: 120]

    -h, --head <HEAD>
            Set color of a head character

            OPTIONS: white, red, blue, green, magenta, cyan, yellow, random, rainbow, r,g,b

            [default: white]

        --help
            Print help information

    -l, --brightness <BRIGHTNESS>
            Set brightness effect for tail

            OPTIONS: none, random, gradient

            [default: random]

    -t, --tail <TAIL>
            Set color of tail characters

            OPTIONS: white, red, blue, green, magenta, cyan, yellow, random, rainbow, r,g,b

            [default: green]

    -V, --version
            Print version information
```

## how to run

- clone this repo then, 

```
cargo run
cargo run -- [OPTIONS]
```

- press `q` while running to quit

## reference

- terminal backend https://crates.io/crates/termion
- CLI arguments https://github.com/clap-rs/clap/blob/v3.1.18/examples/tutorial_derive/README.md
- original source cmatrix https://github.com/abishekvashok/cmatrix/blob/master/cmatrix.c
- termion example https://github.com/redox-os/games/blob/master/src/minesweeper/main.rs
- rust cookbook https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
- ref for how terminal animation is implemented https://github.com/Treeniks/throbber
