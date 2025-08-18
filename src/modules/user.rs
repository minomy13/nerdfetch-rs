use crate::config::{conf_unwrap_or, schema::Config};
use crate::modules::module::Module;
use color_print::cformat;
use sysinfo::System;
use users::{get_current_uid, get_user_by_uid};

#[derive(Default)]
pub struct User;

impl Module for User {
    fn get_formatted(&self, _: &Config) -> String {
        cformat!(
            "<s><y!>{}<r!>@<b!>{}",
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
