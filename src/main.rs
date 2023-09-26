use std::{thread::sleep, time::{Duration, self}, process::Command, array, clone, collections::VecDeque};

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

    let text = if args.check_command {
        get_command(&args.text)
    } else {
        args.text.to_owned()
    };

    let mut text_chars: Vec<char> = text.chars().collect();

    if text_chars.len() > length {
        for char in sep.chars() {
            text_chars.push(char)
        }
    }


    let mut display_chars = VecDeque::from(vec!['\0'; length]);
    for char_index in 0..text_chars.len() {
        if char_index >= length {
            break;
        }
        display_chars[char_index] = text_chars[char_index];
        i = char_index;
    }

    loop {
        if args.check_command && last_check.elapsed().as_millis() > args.command_delay {
            let cmd_result_text = get_command(&args.text);
            text_chars = cmd_result_text.chars().collect();

            if text_chars.len() > length {
                for char in sep.chars() {
                    text_chars.push(char)
                }
            }

            last_check = time::Instant::now();
        }

        if length >= text_chars.len() {
            println!("{}", text);
        } else {
            println!();
            for char in display_chars.to_owned() {
                print!("{}", char);
            }

            display_chars.pop_front();

            if i >= text_chars.len() {
                i = 0;
                display_chars.push_back(text_chars[i]);
            } else {
                display_chars.push_back(text_chars[i]);  
            };

            i += 1;
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