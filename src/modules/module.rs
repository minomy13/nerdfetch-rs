use crate::config::schema::Config;

pub trait Module: Default {
    fn get_formatted(&self, config: &Config) -> String {
        format!(
            "\x1b[{}m{}  \x1b[{}m{}\x1b[39m",
            config.theme.icon_color.to_owned() as u32,
            self.get_icon(config),
            config.theme.info_color.to_owned() as u32,
            self.get_info(config)
        )
    }

    fn get_icon(&self, config: &Config) -> String;
    fn get_info(&self, config: &Config) -> String;
}
