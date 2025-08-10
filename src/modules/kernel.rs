use crate::config::conf_unwrap_or;
use crate::config::schema::Config;
use crate::modules::module::Module;
use sysinfo::System;

#[derive(Default)]
pub struct Kernel;

impl Module for Kernel {
    fn get_icon(&self, _: &Config) -> String {
        String::from("")
    }

    fn get_info(&self, config: &Config) -> String {
        let show_name = conf_unwrap_or!(config, false, modules / kernel / show_name);

        if show_name {
            System::kernel_long_version()
        } else {
            System::kernel_version().unwrap()
        }
    }
}
