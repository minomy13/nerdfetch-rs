use crate::config::conf_unwrap_or;
use crate::config::schema::Config;
use crate::modules::module::Module;
use chrono::Duration;
use sysinfo::System;

#[derive(Default)]
pub struct Uptime;

impl Module for Uptime {
    fn get_icon(&self, _: &Config) -> String {
        String::from("󰅶")
    }

    fn get_info(&self, _: &Config) -> String {
        let duration = Duration::new(System::uptime() as i64, 0).unwrap();
        let days = duration.num_days();
        let hours = duration.num_hours() - days * 24;
        let minutes = duration.num_minutes() - days * 24 * 60 - hours * 60;

        format!(
            "{}{}{}",
            if days > 0 {
                format!("{days} days{}", if hours > 0 { ", " } else { "" })
            } else {
                "".to_string()
            },
            if hours > 0 {
                format!("{hours} hours{}", if minutes > 0 { ", " } else { "" })
            } else {
                "".to_string()
            },
            if minutes > 0 {
                format!("{minutes} mins")
            } else {
                "".to_string()
            }
        )
    }

    fn is_active(&self, config: &Config) -> bool {
        conf_unwrap_or!(config, true, modules / uptime / active)
    }
}
