#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

mod events;
mod birthday;
mod providers;

use chrono::{NaiveDate};
use crate::providers::{sqlite::SQLiteProvider};
use crate::providers::{EventProvider, SimpleProvider, my_provider::MyProvider};
use crate::events::{Event, Category};
use std::path::{PathBuf, Path};
use std::fs;
use dirs;
use KT8_SQLite_events::Config;

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
    let provider = SQLiteProvider::new("sqlitedb", Path::new("C:/Users/jerel/AppData/Roaming/today/mydatabase.db"));
    let mut events: Vec<Event> = Vec::new();

    provider.get_events(&mut events);

    for event in &events {
        println!("{}", event);
    }
}