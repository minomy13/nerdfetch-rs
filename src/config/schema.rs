use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme: Option<Theme>,
    pub modules: Option<Modules>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Modules {
    pub alignment: Option<Alignment>,
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
    pub username_color: Option<Color>,
    pub at_color: Option<Color>,
    pub hostname_color: Option<Color>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub ascii_art: Option<String>,
    pub art_color: Option<Color>,
    pub icon_color: Option<Color>,
    pub info_color: Option<Color>,
    pub art_info_spacing: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Alignment {
    Top,
    Center,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Reset = 0,
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
