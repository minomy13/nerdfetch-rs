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
}
