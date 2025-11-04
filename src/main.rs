use std::{env, io::stdin};

use crate::config::{conf_unwrap_or, schema::Alignment};

mod config;
mod modules;

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
        config.theme.get_formatted_ascii_art()
    };

    let modules = modules::get_modules(&config);

    let mut lines = vec![];

    for line in ascii_art.lines() {
        if let Some(color) = &config.theme.art_color {
            lines.push(format!("\x1b[{}m  {line}\x1b[0m", color.to_owned() as u32))
        } else {
            lines.push(format!("  {line}"))
        }
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

    for i in 0..modules.len() {
        match lines.get_mut(i + modules_start) {
            Some(line) => line.push_str(&format!("  {}", modules.get(i).unwrap())),
            None => lines.push(format!(
                "    {}{}",
                " ".repeat(config.theme.get_fmt_art_longest_ln()),
                modules.get(i).unwrap()
            )),
        }
    }

    for line in lines {
        println!("{line}")
    }
}
