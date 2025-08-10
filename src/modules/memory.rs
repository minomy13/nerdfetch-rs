use crate::config::schema::Config;
use crate::modules::module::Module;
use sysinfo::System;

#[derive(Default)]
pub struct Memory;

impl Module for Memory {
    fn get_icon(&self, _: &Config) -> String {
        String::from("󰍛")
    }

    fn get_info(&self, _: &Config) -> String {
        let sys = System::new_all();
        let used = sys.used_memory() as f64;
        let total = sys.total_memory() as f64;
        format!(
            "{:.0}/{:.0} MB ({:.0}%)",
            used / 1_000_000.0,
            total / 1_000_000.0,
            used / total * 100.0
        )
    }
}
