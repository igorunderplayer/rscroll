use std::{thread::sleep, time::{Duration, self}, process::Command, array, clone};

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
    check_command: bool,

    #[arg(long, default_value_t = 2000)]
    command_delay: u128
}

fn main() {
    let args = Args::parse();
    let length = args.length;
    let delay = Duration::from_millis(args.delay);
    let sep = args.separator;

    let mut last_check = time::Instant::now();

    let mut i = 0;

    let mut text = if args.check_command {
        get_command(&args.text)
    } else {
        args.text.to_string()
    };

    text.push_str(&sep);


    let clone_chars: Vec<char> = text.chars().collect();

    let mut display_chars = vec!['\0'; length];
    for char_index in 0..clone_chars.len() {
        if char_index >= length {
            break;
        }
        display_chars[char_index] = clone_chars[char_index];
    }

    loop {
        if args.check_command && last_check.elapsed().as_millis() > args.command_delay {
            text = get_command(&args.text);
            text.push_str(&sep);
            last_check = time::Instant::now();
        }
    
        let chars: Vec<char> = text.chars().collect();

        if length >= chars.len() {
            println!("{}", text);
        } else {
            println!();
            for char in display_chars.to_owned() {
                print!("{}", char);
            }

            let n = &display_chars[1..];
            display_chars = n.to_vec();

            if i >= chars.len() {
                display_chars.push('\0');
                i = 0;
            } else {
                display_chars.push(chars[i]);
                i += 1;
            };
        }

        sleep(delay)
    }
}

fn get_command(input: &str) -> String{
    let yo = Command::new("sh")
        .arg("-c")
        .arg(input)
        .output()
        .unwrap();

    let t = String::from_utf8(yo.stdout).unwrap();
    t.trim().to_string()
}