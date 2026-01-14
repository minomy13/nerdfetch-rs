use std::{env, io::stdin};

use crate::{
    config::{
        conf_unwrap_or,
        default::ASCII_TUX,
        schema::{Alignment, Color},
    },
    util::{format_ascii_art, get_fmt_line_len},
};

mod config;
mod modules;
mod util;

fn main() {
    let config = config::read();

    let args: Vec<String> = env::args().collect();

    let ascii_art = if args.len() > 1 && args.contains(&"--stdin".to_string()) {
        let mut buf = String::new();
        loop {
            let bytes = stdin()
                .read_line(&mut buf)
                .expect("Couldn't read from stdin");
            if bytes == 0 {
                break;
            }
        }
        buf
    } else {
        conf_unwrap_or!(config, ASCII_TUX.to_owned(), theme / ascii_art).to_owned()
    };
    let ascii_art = format_ascii_art(ascii_art);

    let modules = modules::get_modules(&config);

    let mut lines = vec![];

    for line in ascii_art.lines() {
        let color = conf_unwrap_or!(config, Color::Reset, theme / art_color);
        lines.push(format!("\x1b[{}m  {line}\x1b[0m", color.to_owned() as u32));
    }

    let modules_start = match conf_unwrap_or!(config, Alignment::Top, modules / alignment) {
        Alignment::Top => 0,
        Alignment::Center => {
            if modules.len() < ascii_art.lines().count() {
                (ascii_art.lines().count() - modules.len()) / 2
            } else {
                0
            }
        }
    };

    let longest_art_line = get_fmt_line_len(lines.first().unwrap_or(&"".to_string()));
    for i in 0..modules.len() {
        let spacing = conf_unwrap_or!(config, 2, theme / art_info_spacing);

        match lines.get_mut(i + modules_start) {
            Some(line) => line.push_str(&format!(
                "{}{}",
                " ".repeat(spacing),
                modules.get(i).unwrap()
            )),
            None => lines.push(format!(
                "{}{}",
                " ".repeat(spacing + longest_art_line),
                modules.get(i).unwrap()
            )),
        }
    }

    for line in lines {
        println!("{line}")
    }
}
