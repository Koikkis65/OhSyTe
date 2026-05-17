#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use serde::Deserialize;
use std::path::Path;
use std::error::Error;

pub mod events;
pub mod providers;
pub mod filters;

use chrono::{Datelike, Local, NaiveDate};

use events::{Category, Event, MonthDay};
use filters::{EventFilter, FilterBuilder};
use crate::providers::{EventProvider, SimpleProvider};
use crate::providers::{
    sqlite::SQLiteProvider,
};


#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    pub resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec::<ProviderConfig>,
}

fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            /*
            "text" => {
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "csv" => {
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            },
            */
            "sqlite" => {
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    let test_provider = SimpleProvider::new("test");
    providers.push(Box::new(test_provider));

    providers
}

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter) -> Result<(), Box<dyn Error>> {
    let mut events: Vec<Event> = Vec::new();

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    let providers = create_providers(config, config_path);

    let mut count = 0;
    for provider in providers {
        provider.get_events(&mut events);  // polymorphism!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    for event in events {
        if filter.accepts(&event) {
            println!("{}", event);
        }
    }

    Ok(())
}