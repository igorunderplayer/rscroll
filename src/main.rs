use std::{thread::sleep, time::Duration};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    text: String,

    #[arg(short, long, default_value_t = 300)]
    delay: u64,

    #[arg(short, long, default_value_t = 30)]
    length: usize,

    #[arg(short, long, default_value_t = (" - ".to_string()))]
    separator: String
}

fn main() {
    let args = Args::parse();


    let lenght = args.length;
    let text = args.text;
    let delay = Duration::from_millis(args.delay);
    let sep = args.separator;
    let chars: Vec<char> = text.chars().collect();

    let mut i = 0;

    loop {
        if lenght >= chars.len() {
            println!("{}", text);
        } else {
            let should_use_sep: bool = if i + lenght  > chars.len() {
                i = 0;
                true
            } else {false};
            let cut = &chars[i .. lenght + i - should_use_sep as usize * 3];
            println!();

            for char in cut {
                print!("{}", char);
            }

            if should_use_sep {
                print!("{}", sep)
            }
            i += 1;
        }

        sleep(delay)
    }
}
