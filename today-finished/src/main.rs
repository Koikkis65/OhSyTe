#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

// Use library modules from the `today_finished` crate
mod birthday;

use chrono::{Datelike, Local, NaiveDate};
use today_finished::events::{Category, MonthDay};
use today_finished::filters::{EventFilter, FilterBuilder};
use crate::birthday::{get_birthday_from_env, is_birthday_today};
use std::path::{PathBuf, Path};
use std::fs;
use dirs;
use clap::{Args as ClapArgs, Parser, Subcommand};
use today_finished::{add_event_to_provider, list_providers, Config, run};

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
    #[arg(short, long, help = "Event date in DDMM or DD-MM format")]
    date: Option<String>,

    #[arg(short, long, help = "Choose whether or not to comment on birthdays")]
    no_birthday: bool,

    #[arg(short, long, help = "Categories to exclude, comma-separated (a/b, c/d)")]
    exclude_categories: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all event providers
    Providers,
    /// Add an event to a text or CSV provider
    Add(AddArgs),
}

#[derive(ClapArgs)]
struct AddArgs {
    #[arg(short, long, help = "Name of event provider")]
    provider: String,

    #[arg(short, long, help = "Date of event. Format: YYYY-MM-DD")]
    date: String,

    #[arg(short = 'e', long, help = "Description of event")]
    description: String,

    #[arg(short, long, help = "Category of event. Format: primary[/secondary]")]
    category: String,
}

fn main() {
    let args = Args::parse();

    let cfg_path = match get_config_path("today") {
        Some(path) => path,
        None => {
            eprintln!("Unable to get config path");
            return;
        }
    };

    let config_file = cfg_path.join("today.toml");
    let config_contents = match fs::read_to_string(&config_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read config file {}: {}", config_file.display(), e);
            return;
        }
    };

    let config: Config = match toml::from_str(&config_contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse config file {}: {}", config_file.display(), e);
            return;
        }
    };

    match args.command {
        Some(Commands::Providers) => {
            list_providers(&config);
            return;
        }
        Some(Commands::Add(add_args)) => {
            let date = match NaiveDate::parse_from_str(&add_args.date, "%F") {
                Ok(date) => date,
                Err(e) => {
                    eprintln!("Invalid event date '{}': {}", add_args.date, e);
                    return;
                }
            };
            let category = Category::from_str(&add_args.category);
            if let Err(e) = add_event_to_provider(
                &config,
                cfg_path.as_path(),
                &add_args.provider,
                date,
                &add_args.description,
                &category,
            ) {
                eprintln!("{}", e);
                return;
            }
            println!(
                "Added event to '{}' on {}",
                add_args.provider,
                date.format("%F")
            );
            return;
        }
        None => {}
    }

    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md)
    } else {
        let today = Local::now().date_naive();
        MonthDay::new(today.day(), today.month())
    };

    let filters = FilterBuilder::new()
        .month_day(month_day)
        .build();

    
    let exclude_categories = match args.exclude_categories {
        Some(ec) => {
            ec.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
        }
        None => Vec::new()
    };

    if args.no_birthday {
        println!("Birthdays will not be included in the output.");
    }
    else {
        if !is_birthday_today(get_birthday_from_env(None)) {
            println!("No birthday today.");
        }
    }

    run(&config, cfg_path.as_path(), &filters, &exclude_categories).unwrap();

}