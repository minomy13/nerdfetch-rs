use crate::config::conf_unwrap_or;
use crate::config::schema::Config;
use crate::modules::module::Module;
#[cfg(target_os = "linux")]
use regex::Regex;
use sysinfo::System;

#[derive(Default)]
pub struct Os;

impl Module for Os {
    fn get_icon(&self, _: &Config) -> String {
        #[cfg(target_os = "macos")]
        return String::from("");

        #[cfg(target_os = "linux")]
        {
            let default = "".to_string();
            let regex = Regex::new(r"^Linux\s\((\w+)(?:\s(.+))?\)$").unwrap();
            match regex.captures(&System::long_os_version().unwrap_or_default()) {
                Some(distro) => {
                    // unwrap should be no problem due to sysinfo's nature
                    match distro.get(1).unwrap().as_str().to_lowercase().as_str() {
                        "debian" => "".to_string(),
                        "arch" => "󰣇".to_string(),
                        "endeavouros" => "".to_string(),
                        "fedora" => "".to_string(),
                        "gentoo" => "".to_string(),
                        "rhel" => "".to_string(),
                        "slackware" => "".to_string(),
                        "void" => "".to_string(),
                        "alpine" => "".to_string(),
                        "nixos" => "󱄅".to_string(),
                        "artix" => "".to_string(),
                        "exherbo" => "󰆚".to_string(),
                        "mageia" => "".to_string(),
                        "manjaro" => "".to_string(),
                        "opensuse" => "".to_string(),
                        "solus" => "".to_string(),
                        "ubuntu" => "".to_string(),
                        "linuxmint" => "󰣭".to_string(),
                        "trisquel" => "".to_string(),
                        "puppy" => "".to_string(),
                        "coreos" => "".to_string(),
                        "mx" => "".to_string(),
                        "vanilla" => "".to_string(),
                        "pop_os" => "".to_string(),
                        "raspbian" => "".to_string(),
                        "deepin" => "".to_string(),
                        "almalinux" => "".to_string(),
                        "garuda" => "".to_string(),
                        "centos" => "".to_string(),
                        "rocky" => "".to_string(),
                        _ => default,
                    }
                }
                None => default,
            }
        }
    }

    fn get_info(&self, _: &Config) -> String {
        #[cfg(target_os = "macos")]
        return System::long_os_version().unwrap_or("n/a".to_string());

        #[cfg(target_os = "linux")]
        {
            let regex = Regex::new(r"Linux\s\(([\w\s./]+)*\)").unwrap();
            match regex.captures(&System::long_os_version().unwrap_or_default()) {
                Some(captures) => {
                    if let Some(version) = captures.get(1) {
                        version.as_str().to_string()
                    } else {
                        "n/a".to_string()
                    }
                }
                None => "n/a".to_string(),
            }
        }
    }

    fn is_active(&self, config: &Config) -> bool {
        conf_unwrap_or!(config, true, modules / os / active)
    }
}
