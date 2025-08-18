use crate::config::conf_unwrap_or;
use crate::config::schema::Config;
use crate::modules::module::Module;
use color_print::cformat;

#[derive(Default)]
pub struct ColorPalette;

impl Module for ColorPalette {
    fn get_icon(&self, _: &Config) -> String {
        String::from("")
    }

    fn get_info(&self, _: &Config) -> String {
        cformat!("<r!>███<g!>███<y!>███<b!>███<m!>███<c!>███")
    }

    fn is_active(&self, config: &Config) -> bool {
        conf_unwrap_or!(config, true, modules / color_palette / active)
    }
}
