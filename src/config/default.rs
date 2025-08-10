use crate::config::schema::{Color, Theme};

impl Default for Theme {
    fn default() -> Self {
        Self {
            ascii_art: String::from(
                r"    ___
   (.. \
   (<> |
  //  \ \
 ( |  | /|
_/\ __)/_)
\/-____\/",
            ),
            art_color: None,
            icon_color: Color::BrightMagenta,
            info_color: Color::Green,
        }
    }
}
