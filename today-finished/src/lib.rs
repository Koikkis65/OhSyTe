#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use serde::Deserialize;
use std::path::Path;
use std::error::Error;
use std::io;

pub mod events;
pub mod providers;
pub mod filters;

use chrono::{Datelike, Local, NaiveDate};

use events::{Category, Event, MonthDay};
use filters::{EventFilter, FilterBuilder};
use crate::providers::{EventProvider, SimpleProvider};
use crate::providers::{
    sqlite::SQLiteProvider,
    textfile::TextFileProvider,
    csv::CSVFileProvider
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
            "text" => {
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            
            "csv" => {
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "sqlite" => {
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    providers
}

pub fn list_providers(config: &Config) {
    for provider in config.providers.iter() {
        println!("{} ({}) -> {}", provider.name, provider.kind, provider.resource);
    }
}

pub fn add_event_to_provider(
    config: &Config,
    config_path: &Path,
    provider_name: &str,
    date: NaiveDate,
    description: &str,
    category: &Category,
) -> Result<(), Box<dyn Error>> {
    let provider_config = config
        .providers
        .iter()
        .find(|provider| provider.name == provider_name)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Unknown provider '{}'", provider_name),
            )
        })?;

    let path = config_path.join(&provider_config.resource);
    match provider_config.kind.as_str() {
        "text" => {
            let provider = crate::providers::textfile::TextFileProvider::new(&provider_config.name, &path);
            provider.add_event(date, description, category)
        }
        "csv" => {
            let provider = crate::providers::csv::CSVFileProvider::new(&provider_config.name, &path);
            provider.add_event(date, description, category)
        }
        "sqlite" => Err(Box::new(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("Adding events is not supported for SQL provider '{}'", provider_config.name),
        ))),
        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unable to make provider: {:?}", provider_config),
        ))),
    }
}

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter, exclude_categories: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut events: Vec<Event> = Vec::new();

    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.day(), today.month());

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
        if filter.accepts(&event) && !exclude_categories.contains(&event.category().to_string()) {
            println!("{}", event);
        }
    }

    Ok(())
}