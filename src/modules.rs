use crate::config::schema::Config;
use crate::modules::color_palette::ColorPalette;
use crate::modules::kernel::Kernel;
use crate::modules::memory::Memory;
use crate::modules::module::Module;
use crate::modules::os::Os;
use crate::modules::shell::Shell;
use crate::modules::uptime::Uptime;
use crate::modules::user::User;
use std::string::String;

mod color_palette;
mod kernel;
mod memory;
mod module;
mod os;
mod shell;
mod uptime;
mod user;

macro_rules! formatted_modules_vec {
    ($conf:ident, $($module:ident), *) => {
        vec![$($module::default().get_formatted(&$conf),)+]
    };
}

pub fn get_modules(config: &Config) -> Vec<String> {
    return formatted_modules_vec!(
        config,
        User,
        Os,
        Kernel,
        Memory,
        Shell,
        Uptime,
        ColorPalette
    )
}