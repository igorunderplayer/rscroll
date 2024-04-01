use std::{
    collections::VecDeque,
    process::Command,
    thread::sleep,
    time::{self, Duration},
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    text: String,

    #[arg(short, long, default_value_t = 300)]
    delay: u64,

    #[arg(short, long, default_value_t = 30)]
    length: usize,

    #[arg(short, long, default_value_t = String::from(" - "))]
    separator: String,

    #[arg(short, long, default_value_t = false)]
    check_command: bool,

    #[arg(long, default_value_t = 2000)]
    command_delay: u128,
}

fn main() {
    let args = Args::parse();
    let length = args.length;
    let delay = Duration::from_millis(args.delay);
    let sep = &args.separator;

    let mut last_check = time::Instant::now();

    let text = if args.check_command {
        get_result_from_command(&args.text)
    } else {
        args.text.to_owned()
    };

    let mut last_result = text.clone();
    let mut text_chars: Vec<char> = text.chars().collect();

    if text_chars.len() > length {
        for char in sep.chars() {
            text_chars.push(char)
        }
    }

    let mut display_chars = create_display_chars(length, &text_chars);
    let mut i = display_chars.len();

    loop {
        if args.check_command && last_check.elapsed().as_millis() > args.command_delay {
            let cmd_result_text = get_result_from_command(&args.text);

            if cmd_result_text != last_result {
                let res_chars: Vec<char> = cmd_result_text.chars().collect();

                text_chars = res_chars.clone();

                if res_chars.len() > length {
                    text_chars.extend(sep.chars());
                }

                for i in 0..length {
                    if i >= text_chars.len() {
                        display_chars[i] = '\0';
                    } else {
                        display_chars[i] = text_chars[i];
                    }
                }

                i = display_chars.len();
            }

            last_result = cmd_result_text;
            last_check = time::Instant::now();
        }

        println!();
        if length >= text_chars.len() {
            for char in &text_chars {
                print!("{}", char);
            }
        } else {
            for char in &display_chars {
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

fn get_result_from_command(input: &str) -> String {
    let yo = Command::new("sh").arg("-c").arg(input).output().unwrap();

    let t = String::from_utf8(yo.stdout).unwrap();
    t.trim().to_string()
}

fn create_display_chars(length: usize, text_chars: &[char]) -> VecDeque<char> {
    let mut display_chars = VecDeque::from(vec!['\0'; length]);
    for char_index in 0..text_chars.len() {
        if char_index >= length {
            break;
        }
        display_chars[char_index] = text_chars[char_index];
    }

    display_chars
}
