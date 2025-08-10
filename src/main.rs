mod config;
mod modules;

fn main() {
    let config = config::read();

    let ascii_art = config.theme.get_formatted_ascii_art();
    let modules = modules::get_modules(&config);

    let mut lines = vec![];

    for line in ascii_art.lines() {
        if let Some(color) = &config.theme.art_color {
            lines.push(format!("\x1b[{}m  {line}", color.to_owned() as u32))
        } else {
            lines.push(format!("  {line}"))
        }
    }

    for i in 0..modules.len() {
        match lines.get_mut(i) {
            Some(line) => line.push_str(&format!("  {}", modules.get(i).unwrap())),
            None => lines.push(format!(
                "{}{}",
                " ".repeat(config.theme.get_fmt_art_longest_ln()),
                modules.get(i).unwrap()
            )),
        }
    }

    for line in lines {
        println!("{line}")
    }
}
