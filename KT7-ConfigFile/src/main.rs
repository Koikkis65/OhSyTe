mod events;
mod birthday;
mod providers;
mod lib;

use chrono::{NaiveDate};

use crate::providers::{EventProvider, SimpleProvider, my_provider::MyProvider};
use crate::events::{Event, Category};
use std::path::PathBuf;
use std::fs;
use dirs;
use crate::lib::Config;

fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join(app_name);
        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            }
        }
        return Some(config_path);
    }
    None
}




fn main() {
    const APP_NAME: &str = "today";
    if let Some(config_path) = get_config_path(APP_NAME) {
        let toml_path = config_path.join(format!("{}.toml", APP_NAME));
        println!("Looking for configuration file '{}'", &toml_path.display());
        let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
        let config: Config = toml::from_str(&config_str).expect("valid configuration file");
        println!("config: {:#?}", config);
    }
}
