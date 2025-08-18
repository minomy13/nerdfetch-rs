#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "macos")]
use std::process::Command;

use crate::{
    config::{conf_unwrap_or, schema::Config},
    modules::module::Module,
};

pub struct Host;

impl Module for Host {
    fn get_icon(&self, _: &Config) -> String {
        String::from("")
    }

    fn get_info(&self, _: &Config) -> String {
        #[cfg(target_os = "linux")]
        {
            let product =
                fs::read_to_string("/sys/class/dmi/id/product_name").unwrap_or("n/a".to_string());

            let vendor =
                fs::read_to_string("/sys/class/dmi/id/sys_vendor").unwrap_or("n/a".to_string());

            format!("{} {}", vendor.trim(), product.trim())
        }

        #[cfg(target_os = "macos")]
        {
            let product = match Command::new("sysctl").arg("-n").arg("hw.model").output() {
                Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
                Err(_) => "n/a".to_string(),
            };

            format!("Apple {product}")
        }
    }

    fn is_active(&self, config: &Config) -> bool {
        conf_unwrap_or!(config, false, modules / host / active)
    }
}
