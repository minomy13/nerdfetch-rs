use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme: Theme,
    pub modules: Option<Modules>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Modules {
    pub color_palette: Option<ColorPaletteConfig>,
    pub host: Option<HostConfig>,
    pub kernel: Option<KernelConfig>,
    pub memory: Option<MemoryConfig>,
    pub os: Option<OsConfig>,
    pub shell: Option<ShellConfig>,
    pub uptime: Option<UptimeConfig>,
    pub user: Option<UserConfig>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ColorPaletteConfig {
    pub active: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct HostConfig {
    pub active: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct KernelConfig {
    pub active: Option<bool>,
    pub show_name: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct MemoryConfig {
    pub active: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct OsConfig {
    pub active: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ShellConfig {
    pub active: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct UptimeConfig {
    pub active: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub ascii_art: String,
    pub art_color: Option<Color>,
    pub icon_color: Color,
    pub info_color: Color,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    White = 37,
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
}

impl Theme {
    pub fn get_fmt_art_longest_ln(&self) -> usize {
        let mut longest_line = 0;
        for line in self.ascii_art.lines() {
            let len = line.len();
            if len > longest_line {
                longest_line = len
            }
        }
        longest_line
    }

    pub fn get_formatted_ascii_art(&self) -> String {
        let mut result = String::new();

        let longest_line = self.get_fmt_art_longest_ln();

        for line in self.ascii_art.lines() {
            result.push_str(&format!(
                "{line}{}\n",
                " ".repeat(longest_line - line.len())
            ))
        }

        result
    }
}
