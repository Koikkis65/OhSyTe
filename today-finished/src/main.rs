#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

mod events;
mod birthday;
mod providers;
mod filters;

use chrono::{NaiveDate, Datelike, Local};
use crate::providers::{sqlite::SQLiteProvider};
use crate::providers::{EventProvider, SimpleProvider, my_provider::MyProvider};
use crate::events::{Event, Category, MonthDay};
use crate::filters::{EventFilter, FilterBuilder};
use crate::birthday::{get_birthday_from_env, is_birthday_today};
use std::path::{PathBuf, Path};
use std::fs;
use dirs;
use today_finished::Config;
use clap::Parser;

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

#[derive(Parser)]
#[command(name = "today")]
struct Args {
    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,

    #[arg(short, long, help = "Choose whether or not to comment on birthdays")]
    no_birthday: bool,


}

fn main() {
    let args = Args::parse();
    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md)
    }
    else {
        let today = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };

    if args.no_birthday {
        println!("Birthdays will not be included in the output.");
    }
    else {
        if !is_birthday_today(get_birthday_from_env(None)) {
            println!("No birthday today.");
        }
    }
     
    let filter: EventFilter = FilterBuilder::new()
        .month_day(month_day)
        .build();

    let provider = SQLiteProvider::new("sqlitedb", Path::new("C:/Users/jerel/AppData/Roaming/today/mydatabase.db"));
    let mut events: Vec<Event> = Vec::new();

    provider.get_events(&mut events);


    for event in &events {
        if filter.accepts(event) {
            println!("{}", event);
        }
    }
}