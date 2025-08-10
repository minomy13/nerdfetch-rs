mod default;
pub mod schema;

use crate::config::schema::Config;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::{env, fs};

const CONFIG_FILE: &str = "nerdfetch-rs.toml";

macro_rules! conf_unwrap_or {
    ($conf:ident, $or:expr, $name:ident/$($others:ident)/+) => {
        if let Some($name) = &$conf.$name {
            conf_unwrap_or!($name, $or, $($others)/+)
        } else { $or }
    };

    ($conf:ident, $or:expr, $name:ident) => {
        if let Some($name) = $conf.$name {
            $name
        } else { $or }
    }
}
pub(crate) use conf_unwrap_or;

pub fn read() -> Config {
    match fs::read_to_string(open_config_file().unwrap_or_else(|not_found_err| {
        panic!("Error while reading config: {}", not_found_err.to_string())
    })) {
        Ok(file) => match toml::from_str::<Config>(&file) {
            Ok(result) => result,
            Err(error) => panic!(
                "Error while reading config. Config file not formatted properly: {}",
                error.message()
            ),
        },

        Err(error) => match error.kind() {
            ErrorKind::NotFound => create_config_file().unwrap_or_else(|not_found_err| {
                panic!("Error while reading config: {}", not_found_err.to_string())
            }),
            _ => panic!("Error while reading config: {}", error.to_string()),
        },
    }
}

fn create_config_file() -> Result<Config, Error> {
    let mut new_config_file = File::create_new(open_config_file()?)?;
    let config = toml::to_string(&Config::default()).expect("Can't serialize default config");
    new_config_file.write_all(config.as_bytes())?;

    Ok(Config::default())
}

fn open_config_dir() -> Result<PathBuf, Error> {
    // deprecated because may have unexpected behavior on Windows, but remember that we hate windows
    #[allow(deprecated)]
    let config_dir = match env::home_dir() {
        Some(dir) => dir,
        None => return Err(Error::new(ErrorKind::NotFound, "Home directory not found")),
    }
    .join(r".config");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

fn open_config_file() -> Result<PathBuf, Error> {
    Ok(open_config_dir()?.join(CONFIG_FILE))
}
