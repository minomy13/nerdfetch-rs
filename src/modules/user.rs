use crate::config::schema::Color;
use crate::config::{conf_unwrap_or, schema::Config};
use crate::modules::module::Module;
use sysinfo::System;
use users::{get_current_uid, get_user_by_uid};

pub struct User;

impl Module for User {
    fn get_formatted(&self, config: &Config) -> String {
        let username_color =
            conf_unwrap_or!(config, Color::BrightYellow, modules / user / username_color) as u32;
        let at_color = conf_unwrap_or!(config, Color::BrightRed, modules / user / at_color) as u32;
        let hostname_color =
            conf_unwrap_or!(config, Color::BrightBlue, modules / user / hostname_color) as u32;

        format!(
            "\x1b[1;{username_color}m{}\x1b[{at_color}m@\x1b[{hostname_color}m{}\x1b[0m",
            match get_user_by_uid(get_current_uid()) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => "n/a".to_string(),
            },
            System::host_name().unwrap_or("n/a".to_string())
        )
    }

    fn get_icon(&self, _: &Config) -> String {
        unimplemented!()
    }

    fn get_info(&self, _: &Config) -> String {
        unimplemented!()
    }

    fn is_active(&self, config: &Config) -> bool {
        conf_unwrap_or!(config, true, modules / user / active)
    }
}
