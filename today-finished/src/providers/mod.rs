#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
pub mod my_provider;
pub mod sqlite;
pub mod textfile;
pub mod csv;
use chrono::{NaiveDate, Local};
use std::error::Error;
use std::io;

use crate::events::{Event, Category};
pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
    fn add_event(
        &self,
        _date: NaiveDate,
        _description: &str,
        _category: &Category,
    ) -> Result<(), Box<dyn Error>> {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("Provider '{}' does not support adding events", self.name()),
        )))
    }
}

pub struct SimpleProvider {
    name: String,
}

impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, events: &mut Vec<Event>) {
        events.push(Event::new_singular(
            NaiveDate::parse_from_str("1991-01-17", "%Y-%m-%d").unwrap(), 
            String::from("Operaatio Desert Storm, Yhdysvallat sekoilee"), 
            Category::from_primary("History")
        ));
        events.push(Event::new_singular(
            NaiveDate::parse_from_str("1995-01-18", "%Y-%m-%d").unwrap(), 
            String::from("Suomi liittyy Euroopan unioniin"), 
            Category::from_primary("History")
        ));
    }
}