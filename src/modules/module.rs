use crate::config::schema::Config;

pub trait Module {
    fn get_formatted(&self, config: &Config) -> String {
        format!(
            "\x1b[{}m{}  \x1b[{}m{}\x1b[0m",
            config.theme.icon_color.to_owned() as u32,
            self.get_icon(config),
            config.theme.info_color.to_owned() as u32,
            self.get_info(config)
        )
    }

    fn is_active(&self, config: &Config) -> bool;
    fn get_icon(&self, config: &Config) -> String;
    fn get_info(&self, config: &Config) -> String;
}
