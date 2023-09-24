use std::{thread::sleep, time::Duration, process::Command};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    text: String,

    #[arg(short, long, default_value_t = 300)]
    delay: u64,

    #[arg(short, long, default_value_t = 30)]
    length: usize,

    #[arg(short, long, default_value_t = (" - ".to_string()))]
    separator: String,

    #[arg(short, long, default_value_t = false)]
    check_command: bool
}

fn main() {
    let args = Args::parse();
    let lenght = args.length;
    let delay = Duration::from_millis(args.delay);
    let sep = args.separator;

    let mut i = 0;

    loop {
        let text = if args.check_command {
            let yo = Command::new("sh")
            .arg("-c")
            .arg(&args.text)
            .output()
            .unwrap();
    
            let t = String::from_utf8(yo.stdout).unwrap();
            t.trim().to_owned()
        } else {
            args.text.trim().to_owned()
        };
    
        let chars: Vec<char> = text.chars().collect();


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
