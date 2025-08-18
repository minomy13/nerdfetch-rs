use crate::config::conf_unwrap_or;
use crate::config::schema::Config;
use crate::modules::module::Module;
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

pub struct Shell;

impl Module for Shell {
    fn get_icon(&self, _: &Config) -> String {
        String::from("")
    }

    fn get_info(&self, _: &Config) -> String {
        match get_user_by_uid(get_current_uid()) {
            Some(user) => user.shell().to_string_lossy().to_string(),
            None => "n/a".to_string(),
        }
    }

    fn is_active(&self, config: &Config) -> bool {
        conf_unwrap_or!(config, true, modules / shell / active)
    }
}
