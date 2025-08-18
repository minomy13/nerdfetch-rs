use crate::config::schema::Config;
use crate::modules::color_palette::ColorPalette;
use crate::modules::kernel::Kernel;
use crate::modules::memory::Memory;
use crate::modules::module::Module;
use crate::modules::os::Os;
use crate::modules::shell::Shell;
use crate::modules::uptime::Uptime;
use crate::modules::user::User;

mod color_palette;
mod kernel;
mod memory;
mod module;
mod os;
mod shell;
mod uptime;
mod user;

macro_rules! boxed_vec {
    ($($module:ident), *) => {
        vec![$(Box::new($module),)+]
    };
}

pub fn get_modules(config: &Config) -> Vec<String> {
    let modules: Vec<Box<dyn Module>> =
        boxed_vec![User, Os, Kernel, Memory, Shell, Uptime, ColorPalette];
    modules
        .iter()
        .filter(|module| module.is_active(config))
        .map(|module| module.get_formatted(config))
        .collect()
}
