use crate::{
    conf_unwrap_or,
    config::schema::{Color, Config},
};

pub trait Module {
    fn get_formatted(&self, config: &Config) -> String {
        format!(
            "\x1b[0m\x1b[{}m{}  \x1b[0m\x1b[{}m{}\x1b[0m",
            conf_unwrap_or!(config, Color::BrightMagenta, theme / icon_color) as u32,
            self.get_icon(config),
            conf_unwrap_or!(config, Color::Green, theme / info_color) as u32,
            self.get_info(config)
        )
    }

    fn is_active(&self, config: &Config) -> bool;
    fn get_icon(&self, config: &Config) -> String;
    fn get_info(&self, config: &Config) -> String;
}
